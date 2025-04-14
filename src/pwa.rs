use std::io::{Read, Write};

use include_dir::{Dir, include_dir};

use crate::config::Config;
use std::error::Error;

const PWA_DIR: Dir = include_dir!("pwa");

pub fn init_pwa(config: &Config) -> Result<(), Box<dyn Error>> {
    let pwa_dir = config.root.join("pwa");
    if !pwa_dir.exists() {
        std::fs::create_dir_all(pwa_dir)?;
        PWA_DIR.extract(config.root.join("pwa"))?;
    }

    let index = config.root.join(&config.index);
    info!("PWA: {:?}", index.file_name());
    let mut file = std::fs::File::open(&index)?;
    let mut index_html = String::new();
    file.read_to_string(&mut index_html)?;

    // insert mainfest
    const MANIFEST_LINK: &str =
        "<link crossorigin=\"use-credentials\" rel=\"manifest\" href=\"/pwa/manifest.json\">\n";
    if !index_html.contains(MANIFEST_LINK) {
        const MANIFEST_MARKER: &str = r#"<meta charset="UTF-8" />"#;
        let insert_index = index_html.find(MANIFEST_MARKER).ok_or("marker not find")?;
        index_html.insert_str(insert_index, MANIFEST_LINK);
    }

    //insert serviceWorker
    const SERVICE_INSERT: &str = "    <script>\n        if (typeof navigator.serviceWorker !== 'undefined') {\n            navigator.serviceWorker.register('/pwa/sw.js')\n        }\n        </script>\n";
    if !index_html.contains(SERVICE_INSERT) {
        const SERVICE_MARKER: &str = "<body>\n\t<div id=\"init-screen\">";
        const MARKER_PREFIX_LEN: usize = "<body>\n\t".len();

        let insert_index_base = index_html
            .find(SERVICE_MARKER)
            .ok_or("Failed to find marker in index.html")?;
        let insert_index = insert_index_base + MARKER_PREFIX_LEN;
        index_html.insert_str(insert_index, SERVICE_INSERT);
    }

    let mut file = std::fs::File::create(&index)?;
    file.write_all(index_html.as_bytes())?;

    Ok(())
}
