use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{auth::User, Cfg};

#[derive(Debug, Deserialize)]
pub struct Save {
    pub slot: u32,
    pub name: String,
    pub save: String,
    pub story: String,
    pub new: bool,
}

#[instrument(skip(save, state))]
pub async fn save(
    State(state): State<Cfg>,
    User(user): User,
    Json(Save {
        slot,
        name,
        save,
        story,
        new,
    }): Json<Save>,
) -> (StatusCode, Json<&'static str>) {
    let save_dir = state.save_dir.join(user);
    debug!(?save_dir, "存档目录");

    if let Err(error) = tokio::fs::create_dir_all(&save_dir).await {
        const MSG: &str = "创建存档目录失败";
        warn!(%error, ?save_dir, "{MSG}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG));
    }

    let file_name = format!("{}-{name}-{slot:02}.save", if new { "00" } else { "01" });
    let save_path = save_dir.join(file_name);

    debug!("开始压缩存档数据");
    let mut data = lz_str::compress_to_base64(&save);
    data += &lz_str::compress_to_base64(&format!("\"{}\": {}", story, data.len()));
    debug!("存档压缩结束");

    if let Err(error) = tokio::fs::write(&save_path, data).await {
        const MSG: &str = "存档文件保存失败";
        warn!(%error, ?save_path, "{MSG}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG));
    }

    const MSG: &str = "存档保存成功";
    info!(?save_path, "{MSG}");

    (StatusCode::OK, Json(MSG))
}
