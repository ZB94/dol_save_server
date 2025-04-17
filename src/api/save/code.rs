use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{Cfg, api::User};

/// 获取存档码
#[instrument(skip(state))]
pub async fn code(
    State(state): State<Cfg>,
    Extension(User(user)): Extension<User>,
    Path(name): Path<String>,
) -> (StatusCode, Json<String>) {
    let save_path = state.save_dir.join(user).join(name);
    debug!(?save_path, "存档路径");

    if save_path.exists() {
        match tokio::fs::read_to_string(&save_path).await {
            Ok(code) => (StatusCode::OK, Json(code)),
            Err(error) => {
                const MSG: &str = "读取存档文件失败";
                error!(%error, ?save_path, "{MSG}");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG.to_string()))
            }
        }
    } else {
        const MSG: &str = "存档文件不存在";
        debug!(?save_path, "{MSG}");
        (StatusCode::NOT_FOUND, Json(MSG.to_string()))
    }
}
