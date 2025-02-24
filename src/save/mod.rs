mod code;
mod list;
mod remove;
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
        // 获取/删除存档
        .route("/api/save/{name}", get(code::code).delete(remove::remove))
        // 存档列表页面
        .route("/saves", get(list::page))
}
