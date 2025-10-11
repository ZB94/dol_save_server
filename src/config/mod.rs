use std::{error::Error, fmt, path::Path};

use educe::Educe;
use serde::Deserialize;

use backup::Backup;
use game::Game;
use server::Server;

pub mod backup;
pub mod game;
pub mod server;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// 游戏配置
    pub game: Vec<Game>,
    /// 服务配置
    pub server: Server,
    /// 用户认证
    #[serde(default)]
    pub auth: Auth,
    /// 存档备份配置
    #[serde(default)]
    pub backup: Backup,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Auth {
    /// 是否启用
    pub enable: bool,
    /// 是否所有页面都需要登入才能查看
    ///
    /// - 为`true`时`除登录和`PWA`的外其他请求都需要登入
    /// - 为`false`是仅`/api/`开头的请求需要登入
    #[serde(default)]
    pub global: bool,
    /// 用户列表
    #[serde(default)]
    pub users: Vec<User>,
}

/// 认证用户信息
#[derive(Educe, Deserialize, Clone)]
#[educe(Debug)]
pub struct User {
    /// 用户名
    pub username: String,
    /// 密码
    #[educe(Debug(method(fmt_hide)))]
    pub password: String,
}

impl Config {
    /// 默认加载的存档路径
    ///
    /// 可通过环境变量`DOL_SAVE_SERVER`修改
    pub const PATH: &str = "./dol_save_server.toml";

    /// 默认存档内容
    pub const DEFAULT: &str = include_str!("../../dol_save_server_example.toml");

    /// 加载配置
    pub async fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Path::new(
            &std::env::var("DOL_SAVE_SERVER").unwrap_or_else(|_| Config::PATH.to_string()),
        )
        .to_path_buf();

        if !config_path.exists() {
            info!("配置文件不存在, 生成默认配置");
            tokio::fs::write(&config_path, Config::DEFAULT)
                .await
                .inspect_err(|error| error!(%error, "生成默认配置文件失败"))?;
        }

        let config = tokio::fs::read_to_string(&config_path)
            .await
            .inspect_err(|error| error!(%error, ?config_path, "读取配置文件失败"))?;

        let config = toml::from_str::<Config>(&config)
            .inspect_err(|error| error!(%error, ?config_path, "解析配置文件失败"))?;

        info!(?config, "当前配置");

        Ok(config)
    }
}

pub fn fmt_hide<D>(_d: &D, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("***")
}
