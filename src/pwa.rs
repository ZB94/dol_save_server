use axum::{Json, Router, extract::State, routing::get};

use crate::Cfg;
use std::error::Error;

pub fn init_pwa(mut router: Router<Cfg>) -> Result<Router<Cfg>, Box<dyn Error>> {
    router = router.route("/api/pwa/enabled", get(enabled));

    Ok(router)
}

pub async fn enabled(State(state): State<Cfg>) -> Json<bool> {
    Json(state.pwa.enable)
}
