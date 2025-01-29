#[macro_use]
extern crate tracing;

use std::{error::Error, sync::Arc};

mod args;
mod save;

use args::Args;
use axum::{routing::post, Router};
use clap::Parser;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{fmt::time::ChronoLocal, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    init_log();
    info!(?args, "当前参数");

    let index = args.root.join(args.index);
    let root = args.root;

    let router = Router::new()
        .route(
            "/api/save",
            post(save::save).with_state(Arc::new(args.save_dir)),
        )
        .route_service("/", ServeFile::new(index))
        .fallback_service(ServeDir::new(root));

    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .inspect_err(|error| error!(%error, "监听服务地址失败"))?;

    let addr = listener.local_addr()?;
    info!("服务地址: http://{addr}/");

    axum::serve(listener, router)
        .await
        .inspect_err(|error| error!(%error, "服务启动失败"))?;

    Ok(())
}

fn init_log() {
    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::rfc_3339())
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new(format!("{}=trace,warn", env!("CARGO_CRATE_NAME")))
            }),
        )
        .init();
}
