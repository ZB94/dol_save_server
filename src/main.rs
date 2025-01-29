#[macro_use]
extern crate tracing;

use std::{
    error::Error,
    path::{Path, PathBuf},
    sync::Arc,
};

mod args;
mod save;

use args::Args;
use axum::{
    body::Body,
    extract::State,
    response::Response,
    routing::{get, post},
    Router,
};
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

    if !args.no_init_mod {
        init_mod(&root)?;
    }

    let router = Router::new()
        // 保存存档
        .route("/api/save", post(save::save))
        // 显示已有存档
        .route("/saves", get(save_list))
        .with_state(Arc::new(args.save_dir.clone()))
        // 获取存档内容
        .nest_service("/save", ServeDir::new(args.save_dir))
        // 主页
        .route_service("/", ServeFile::new(index))
        // 其他文件
        .fallback_service(ServeDir::new(root));

    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .inspect_err(|error| error!(%error, "监听服务地址失败"))?;

    let addr = listener.local_addr()?;
    info!("服务地址: http://{addr}/");
    info!("你可以访问 http://{addr}/saves 来查看服务端已保存的存档");

    axum::serve(listener, router)
        .await
        .inspect_err(|error| error!(%error, "服务启动失败"))?;

    Ok(())
}

async fn save_list(State(save_dir): State<Arc<PathBuf>>) -> Response<Body> {
    const TEMPLATE: &str = include_str!("../html/savelist.html");
    let mut list = vec![];
    if save_dir.exists() {
        if let Ok(mut files) = tokio::fs::read_dir(save_dir.as_path()).await {
            while let Ok(Some(file)) = files.next_entry().await {
                let path = file.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "save") {
                    let name = file.file_name().to_string_lossy().to_string();
                    list.push(format!(r#"<option value="{name}">{name}</option>"#));
                }
            }
        }
    }
    let list: String = list.join("");

    Response::builder()
        .status(200)
        .header("ContentType", "text/html")
        .body(TEMPLATE.replace("{list}", &list).into())
        .unwrap()
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

#[instrument]
pub fn init_mod(dir: &Path) -> std::io::Result<()> {
    const MOD_LIST_NAME: &str = "modList.json";
    const MOD_NAME: &str = "save_server.mod.zip";
    const MOD_DATA: &[u8] = include_bytes!("../save_server.mod.zip");

    info!("开始初始化模组");

    let mod_list_path = dir.join(MOD_LIST_NAME);
    let mut mod_list = if mod_list_path.exists() {
        let s = std::fs::read_to_string(&mod_list_path)
            .inspect_err(|error| error!(%error, ?mod_list_path, "读取模组列表败"))?;
        serde_json::from_str::<Vec<String>>(&s)
            .inspect_err(|error| error!(%error, ?mod_list_path, "反序列化模组列表失败"))?
    } else {
        vec![]
    };
    info!("已获取模组列表");

    let mut append = true;
    let mod_path = mod_list
        .iter()
        .find_map(|s| {
            let p = Path::new(s);
            if p.file_name().is_some_and(|f| f == MOD_NAME) {
                append = false;
                Some(dir.join(p))
            } else {
                None
            }
        })
        .unwrap_or_else(|| Path::new(dir).join("mod").join(MOD_NAME));
    debug!(?mod_path, "模组路径");

    if let Some(mod_dir) = mod_path.parent() {
        std::fs::create_dir_all(&mod_dir)
            .inspect_err(|error| error!(%error, ?mod_dir, "创建模组目录失败"))?;
        info!(?mod_dir, "已创建模组目录");
    }

    std::fs::write(&mod_path, MOD_DATA)
        .inspect_err(|error| error!(%error, ?mod_path, "保存模组失败"))?;
    info!(?mod_path, "已保存模组");

    if append {
        let p = mod_path
            .strip_prefix(dir)
            .unwrap()
            .to_string_lossy()
            .replace("\\", "/");

        mod_list.push(p);
        let json = serde_json::to_string_pretty(&mod_list)
            .inspect_err(|error| error!(%error, ?mod_list, "序列化模组列表失败"))?;
        std::fs::write(&mod_list_path, &json)
            .inspect_err(|error| error!(%error, ?mod_list_path, %json, "保存模组列表失败"))?;
        info!(?mod_list_path, "已更新模组列表");
    }

    info!("模组初始化结束");

    Ok(())
}
