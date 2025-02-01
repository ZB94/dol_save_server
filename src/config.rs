use std::{fmt, net::SocketAddr, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(default)]
    pub auth: Auth,
    #[serde(default)]
    pub tls: Tls,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Auth {
    pub enable: bool,
    #[serde(default)]
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Tls {
    pub enable: bool,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub cert: String,
}

impl Config {
    pub const PATH: &str = "./dol_save_server.toml";
    pub const DEFAULT: &str = include_str!("../dol_save_server.toml");
}

impl fmt::Debug for Tls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tls")
            .field("enable", &self.enable)
            .field("key", &"***")
            .field("cert", &"***")
            .finish()
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username)
            .field("password", &"***")
            .finish()
    }
}
