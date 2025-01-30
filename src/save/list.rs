use std::{path::PathBuf, sync::Arc};

use axum::{body::Body, extract::State, response::Response};

pub async fn save_list(State(save_dir): State<Arc<PathBuf>>) -> Response<Body> {
    const TEMPLATE: &str = include_str!("../../html/savelist.html");
    let mut list = vec![];
    if save_dir.exists() {
        if let Ok(mut files) = tokio::fs::read_dir(save_dir.as_path()).await {
            while let Ok(Some(file)) = files.next_entry().await {
                let path = file.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "save") {
                    let name = file.file_name().to_string_lossy().to_string();
                    list.push(format!(r#"<option value="{name}">{name}</option>"#));
                }
            }
        }
    }
    let list: String = list.join("");

    Response::builder()
        .status(200)
        .header("ContentType", "text/html")
        .body(TEMPLATE.replace("{list}", &list).into())
        .unwrap()
}
