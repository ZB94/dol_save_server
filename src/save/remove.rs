use axum::extract::{Path, State};

use crate::{auth::User, Cfg};

/// 删除存档
#[instrument(skip(state))]
pub async fn remove(
    State(state): State<Cfg>,
    User(user): User,
    Path(name): Path<String>,
) -> String {
    let save_path = state.save_dir.join(user).join(&name);
    debug!(?save_path, "存档路径");

    if save_path.exists() && save_path.is_file() {
        match tokio::fs::remove_file(save_path).await {
            Err(error) => {
                const MSG: &str = "存档删除失败";
                error!(%error, "{MSG}");
                MSG.to_string()
            }
            _ => {
                info!("存档已删除");
                format!("存档 {name} 已删除")
            }
        }
    } else {
        const MSG: &str = "存档文件不存在";
        debug!(?save_path, "{MSG}");
        MSG.to_string()
    }
}
