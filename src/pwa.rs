use axum::{Json, Router, extract::State, routing::get};
use include_dir::{Dir, include_dir};
use tower_http::services::ServeFile;

use crate::{Cfg, config::Config};
use std::error::Error;

const PWA_DIR: Dir = include_dir!("pwa");

pub fn init_pwa(mut router: Router<Cfg>, config: &Config) -> Result<Router<Cfg>, Box<dyn Error>> {
    router = router.route("/api/pwa/enabled", get(enabled));

    if config.pwa.enable {
        let pwa_dir = config.root.join("pwa");
        if !pwa_dir.exists() {
            std::fs::create_dir_all(&pwa_dir)?;
            PWA_DIR.extract(config.root.join("pwa"))?;
        }

        for file in PWA_DIR.files() {
            let p = file.path();
            if let Some(n) = p.file_name() {
                let url_path = format!("/{}", n.to_string_lossy());
                let file_path = pwa_dir.join(n);

                debug!(url_path, ?file_path, "add pwa route");
                router = router.route_service(&url_path, ServeFile::new(file_path));
            }
        }
    }

    Ok(router)
}

pub async fn enabled(State(state): State<Cfg>) -> Json<bool> {
    Json(state.pwa.enable)
}
