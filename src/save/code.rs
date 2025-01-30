use axum::extract::{Path, State};

use crate::auth::User;

#[instrument(skip(state))]
pub async fn code(
    State(state): State<crate::State>,
    User(user): User,
    Path(name): Path<String>,
) -> String {
    let save_path = state.save_dir.join(user).join(name);
    debug!(?save_path, "存档路径");

    if save_path.exists() {
        match tokio::fs::read_to_string(&save_path).await {
            Ok(code) => code,
            Err(error) => {
                const MSG: &str = "读取存档文件失败";
                error!(%error, ?save_path, "{MSG}");
                MSG.to_string()
            }
        }
    } else {
        const MSG: &str = "存档文件不存在";
        debug!(?save_path, "{MSG}");
        MSG.to_string()
    }
}
