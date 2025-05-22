use std::{
    error::Error,
    fmt,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use educe::Educe;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Deserializer};

use backup::Backup;

pub mod backup;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// 游戏根目录
    pub root: PathBuf,
    /// 访问"/"时的默认文件名
    pub index: String,
    /// 服务地址
    pub bind: SocketAddr,
    /// 存档保存目录
    pub save_dir: PathBuf,
    /// 启动时跳过初始化模组流程
    pub init_mod: bool,
    /// 是否允许存档相关接口跨域访问
    ///
    /// **注意:** 若该功能和`auth`同时启用, 则`tls`功能也需要同步启用才能正常访问跨域请求
    #[serde(default)]
    pub cors: bool,
    /// 访问黑名单 参数为正则表达式
    #[serde(default = "default_blacklist", deserialize_with = "de_blacklist")]
    pub blacklist: Vec<Regex>,
    /// 用户认证
    #[serde(default)]
    pub auth: Auth,
    /// TLS 配置
    #[serde(default)]
    pub tls: Tls,
    /// PWA 配置
    #[serde(default)]
    pub pwa: Pwa,
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

#[derive(Deserialize, Clone, Default, Educe)]
#[educe(Debug)]
pub struct Tls {
    pub enable: bool,
    #[serde(default)]
    #[educe(Debug(method(fmt_hide)))]
    pub key: String,
    #[serde(default)]
    #[educe(Debug(method(fmt_hide)))]
    pub cert: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Pwa {
    pub enable: bool,
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

fn de_blacklist<'de, D>(d: D) -> Result<Vec<Regex>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|s| {
            RegexBuilder::new(&s)
                .case_insensitive(true)
                .build()
                .map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"Regex")
                })
        })
        .collect::<Result<Vec<_>, D::Error>>()
}

fn default_blacklist() -> Vec<Regex> {
    vec![
        RegexBuilder::new(r#".*\.toml"#)
            .case_insensitive(true)
            .build()
            .unwrap(),
    ]
}
