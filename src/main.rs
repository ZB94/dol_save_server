#[macro_use]
extern crate tracing;

use std::{error::Error, path::PathBuf, sync::Arc};

mod args;

use args::Args;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use clap::Parser;
use serde::Deserialize;
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
        .route("/api/save", post(save).with_state(Arc::new(args.save_dir)))
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

#[derive(Debug, Deserialize)]
pub struct Save {
    pub slot: u32,
    pub name: String,
    pub data: String,
}

#[instrument(skip(data, save_dir))]
async fn save(
    State(save_dir): State<Arc<PathBuf>>,
    Json(Save { slot, name, data }): Json<Save>,
) -> (StatusCode, Json<&'static str>) {
    if let Err(error) = tokio::fs::create_dir_all(save_dir.as_ref()).await {
        const MSG: &str = "创建存档目录失败";
        warn!(%error, ?save_dir, "{MSG}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG));
    }

    let file_name = format!("{name}-{slot:02}.save");
    let save_path = save_dir.join(file_name);
    if let Err(error) = tokio::fs::write(&save_path, data).await {
        const MSG: &str = "存档文件保存失败";
        warn!(%error, ?save_path, "{MSG}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(MSG));
    }

    const MSG: &str = "存档保存成功";
    info!(?save_path, "{MSG}");

    (StatusCode::OK, Json(MSG))
}
