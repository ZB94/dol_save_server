use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{Cfg, api::User};

/// 删除存档
#[instrument(skip(state))]
pub async fn remove(
    State(state): State<Cfg>,
    Extension(User(user)): Extension<User>,
    Path(name): Path<String>,
) -> (StatusCode, Json<String>) {
    let save_path = state.save_dir.join(user).join(&name);
    debug!(?save_path, "存档路径");

    if save_path.exists() && save_path.is_file() {
        match tokio::fs::remove_file(save_path).await {
            Err(error) => {
                const MSG: &str = "存档删除失败";
                error!(%error, "{MSG}");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG.to_string()))
            }
            _ => {
                info!("存档已删除");
                (StatusCode::OK, Json(format!("存档 {name} 已删除")))
            }
        }
    } else {
        const MSG: &str = "存档文件不存在";
        debug!(?save_path, "{MSG}");
        (StatusCode::NOT_FOUND, Json(MSG.to_string()))
    }
}
