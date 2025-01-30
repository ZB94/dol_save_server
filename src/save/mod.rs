mod list;
#[allow(clippy::module_inception)]
mod save;

use axum::{
    routing::{get, post},
    Router,
};

use list::save_list;
use tower_http::services::ServeDir;

pub fn router(state: crate::State) -> Router<crate::State> {
    Router::new()
        // 保存存档
        .route("/api/save", post(save::save))
        // 显示已有存档
        .route("/saves", get(save_list))
        // 获取存档内容
        .nest_service("/save", ServeDir::new(&state.save_dir))
}
