use axum::{extract::State, http::StatusCode, Json};
use base64::Engine;
use serde::Deserialize;

use crate::{auth::User, Cfg};

/// 存档信息
#[derive(Debug, Deserialize)]
pub struct Save {
    /// 存档槽位
    ///
    /// `0`表示自动存档槽位
    pub slot: u32,
    /// 存档名称
    pub name: String,
    /// 实际存档内容
    pub save: String,
    /// 游戏名称
    pub story: String,
    /// 存档方式
    ///
    /// 只影响存档名称
    ///
    /// `true`: IndexedDB
    /// `false`: 浏览器本地存储
    pub new: bool,
}

/// 保存存档
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
    let mut data = base64::prelude::BASE64_STANDARD.encode(lz_str::compress_to_uint8_array(&save));
    debug!(data_len = data.len(), "存档数据压缩完成");
    let metadata = format!("{{ \"{}\": {} }}", story, data.len());
    debug!(?metadata, "开始压缩元数据");
    data += &base64::prelude::BASE64_STANDARD.encode(lz_str::compress_to_uint8_array(&metadata));
    debug!("元数据压缩完成");

    if let Err(error) = tokio::fs::write(&save_path, data).await {
        const MSG: &str = "存档文件保存失败";
        warn!(%error, ?save_path, "{MSG}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG));
    }

    const MSG: &str = "存档保存成功";
    info!(?save_path, "{MSG}");

    (StatusCode::OK, Json(MSG))
}
