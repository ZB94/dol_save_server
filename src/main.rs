#[macro_use]
extern crate tracing;

use std::{error::Error, path::Path, sync::Arc};

use axum::{
    Router,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_server::tls_rustls::{RustlsAcceptor, RustlsConfig};
use config::Config;
use path_absolutize::Absolutize;
use tokio::time::MissedTickBehavior;
use tower::{ServiceBuilder, service_fn};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer, cookie::SameSite};
use tracing_subscriber::{EnvFilter, fmt::time::ChronoLocal};

mod api;
mod backup;
mod config;
mod web;

pub type Cfg = Arc<Config>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_log();

    let mut config = Config::load().await?;
    config.save_dir = config
        .save_dir
        .absolutize()
        .inspect_err(|error| error!(%error, "将存档目录转为绝对路径失败"))?
        .to_path_buf();

    let index = config.root.join(&config.index);
    let root = config.root.clone();
    let mut app = Router::new();

    if config.init_mod {
        init_mod(&root)?;
    }

    let cfg = Cfg::new(config);

    app = app
        // API 接口
        .nest("/api", api::route(cfg.clone()))
        // 主页
        .route_service("/", ServeFile::new(index))
        // 其他文件
        .fallback_service(ServeDir::new(root).fallback(service_fn(web::web_service)));

    // Session
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(if cfg.cors {
            SameSite::None
        } else {
            SameSite::Strict
        })
        .with_secure(cfg.cors || cfg.tls.enable)
        .with_expiry(Expiry::OnSessionEnd);

    let app: Router<()> = app
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn_with_state(
                    cfg.clone(),
                    web::blacklist_layer,
                ))
                .layer(session_layer)
                .layer(axum::middleware::from_fn_with_state(
                    cfg.clone(),
                    web_auth_layer,
                ))
                .layer(
                    CompressionLayer::new()
                        .br(true)
                        .deflate(true)
                        .gzip(true)
                        .zstd(true),
                ),
        )
        .with_state(cfg.clone());

    let listener = std::net::TcpListener::bind(&cfg.bind)
        .inspect_err(|error| error!(%error, "监听服务地址失败"))?;

    let addr = listener.local_addr().unwrap();

    if cfg.backup.enable {
        let mut interval = tokio::time::interval(cfg.backup.period);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let cfg = cfg.clone();

        tokio::spawn(async move {
            let mut default_mod = cfg.backup.backup_on_start;
            loop {
                interval.tick().await;
                backup::backup(cfg.clone(), default_mod).await;
                default_mod = false;
            }
        });
    }

    if cfg.tls.enable {
        let tls = RustlsConfig::from_pem(
            cfg.tls.cert.clone().into_bytes(),
            cfg.tls.key.clone().into_bytes(),
        )
        .await
        .inspect_err(|error| error!(%error, "初始化TLS配置失败"))?;

        info!("服务地址: https://{addr}/");
        info!("你可以访问 https://{addr}/saves 来查看服务端已保存的存档");

        let acceptor = RustlsAcceptor::new(tls);
        axum_server::from_tcp(listener)
            .acceptor(acceptor)
            .serve(app.into_make_service())
            .await
            .inspect_err(|error| error!(%error, "服务启动失败"))?;
    } else {
        info!("服务地址: http://{addr}/");
        info!("你可以访问 http://{addr}/saves 来查看服务端已保存的存档");

        axum_server::from_tcp(listener)
            .serve(app.into_make_service())
            .await
            .inspect_err(|error| error!(%error, "服务启动失败"))?;
    }

    Ok(())
}

fn init_log() {
    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::rfc_3339())
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new(format!("{}=info,warn", env!("CARGO_CRATE_NAME")))
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
        std::fs::create_dir_all(mod_dir)
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

#[instrument(skip_all)]
pub async fn web_auth_layer(
    State(cfg): State<Cfg>,
    session: Session,
    request: Request,
    next: Next,
) -> Response {
    const WHITE_LIST: &[&str] = &[
        // 登录相关
        "/login.html",
        // PWA 相关
        "sw.js",
        "/pwa/icon.png",
        "/pwa/manifest.json",
    ];
    const API_PREFIX: &str = "/api/";

    let user = session
        .get::<String>(api::User::SESSION_KEY)
        .await
        .unwrap_or_default();

    let path = request.uri().path();
    let is_global = cfg.auth.global;
    let is_api = path.starts_with(API_PREFIX);

    debug!(path, is_global, is_api, ?user, "auth");

    if !cfg.auth.enable || !is_global || is_api || WHITE_LIST.contains(&path) || user.is_some() {
        debug!(uri = %request.uri(), "鉴权通过");

        return next.run(request).await;
    }

    Redirect::temporary("/login.html").into_response()
}
