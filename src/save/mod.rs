mod code;
mod list;
#[allow(clippy::module_inception)]
mod save;

use axum::{
    routing::{get, post},
    Router,
};

use list::save_list;

use crate::Cfg;

pub fn router() -> Router<Cfg> {
    Router::new()
        // 保存存档
        .route("/api/save", post(save::save))
        // 显示已有存档
        .route("/saves", get(save_list))
        // 获取存档内容
        .route("/save/{name}", get(code::code))
}
