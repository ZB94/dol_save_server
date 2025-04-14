use std::io::{Read, Write};

use crate::config::Config;
use std::error::Error;

pub fn init_pwa(config: &Config) -> Result<(), Box<dyn Error>> {
    let index = config.root.join(&config.index);
    info!("PWA: {:?}", index.file_name());
    let mut file = std::fs::File::open(&index)?;
    let mut index_html = String::new();
    file.read_to_string(&mut index_html)?;

    // insert mainfest
    let manifest_marker = "<meta charset=\"UTF-8\" />";
    let insert_index = index_html.find(manifest_marker).ok_or("marker not find")?;

    //去掉第一个字符 '.'
    let manifest = config
        .pwa
        .source
        .strip_prefix('.')
        .unwrap_or(&config.pwa.source);

    let manifest_link = format!(
        "<link rel=\"manifest\" href=\"{}/manifest.json\">\n",
        manifest
    );
    index_html.insert_str(insert_index, &manifest_link);

    //insert serviceWorker
    let service_marker = "<body>\n\t<div id=\"init-screen\">";
    let service_insert = "    <script>\n        if (typeof navigator.serviceWorker !== 'undefined') {\n            navigator.serviceWorker.register('sw.js')\n        }\n    </script>\n";
    if !index_html.contains(service_insert) {
        let marker_prefix_len = "<body>\n\t".len();
        let insert_index_base = index_html
            .find(service_marker)
            .ok_or("Failed to find marker in index.html")?;
        let insert_index = insert_index_base + marker_prefix_len;
        index_html.insert_str(insert_index, service_insert);
    }

    let mut file = std::fs::File::create(&index)?;
    file.write_all(index_html.as_bytes())?;

    Ok(())
}
