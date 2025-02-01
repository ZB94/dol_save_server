mod code;
mod list;
#[allow(clippy::module_inception)]
mod save;

use axum::{
    routing::{get, post},
    Router,
};

use crate::Cfg;

pub fn router() -> Router<Cfg> {
    Router::new()
        // 保存存档/存档列表
        .route("/api/save", post(save::save).get(list::list))
        // 获取存档内容
        .route("/api/save/{name}", get(code::code))
        // 存档列表页面
        .route("/saves", get(list::page))
}
