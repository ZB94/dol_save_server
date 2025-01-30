use axum::{body::Body, extract::State, response::Response};
use chrono::TimeZone;

pub async fn save_list(State(state): State<crate::State>) -> Response<Body> {
    const TEMPLATE: &str = include_str!("../../html/savelist.html");
    let mut list = vec![];
    if state.save_dir.exists() {
        if let Ok(mut files) = tokio::fs::read_dir(&state.save_dir).await {
            while let Ok(Some(file)) = files.next_entry().await {
                let path = file.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "save") {
                    let time = path
                        .metadata()
                        .and_then(|m| m.modified())
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .and_then(|d| chrono::Local.timestamp_opt(d.as_secs() as i64, 0).single())
                        .map(|time| time.format(" %F %T").to_string())
                        .unwrap_or_default();

                    let name = file.file_name().to_string_lossy().to_string();
                    list.push(format!(r#"<option value="{name}">{name}{time}</option>"#));
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
