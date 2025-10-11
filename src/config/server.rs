use std::net::SocketAddr;

use educe::Educe;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// 服务地址
    pub bind: SocketAddr,
    /// 是否允许存档相关接口跨域访问
    ///
    /// **注意:** 若该功能和`auth`同时启用, 则`tls`功能也需要同步启用才能正常访问跨域请求
    #[serde(default)]
    pub cors: bool,
    /// 访问黑名单 参数为正则表达式
    #[serde(default = "default_blacklist", deserialize_with = "de_blacklist")]
    pub blacklist: Vec<Regex>,
    /// TLS 配置
    #[serde(default)]
    pub tls: Tls,
    /// 是否启用 PWA
    #[serde(default)]
    pub pwa_enabled: bool,
}

#[derive(Deserialize, Clone, Default, Educe)]
#[educe(Debug)]
pub struct Tls {
    pub enable: bool,
    #[serde(default)]
    #[educe(Debug(method(super::fmt_hide)))]
    pub key: String,
    #[serde(default)]
    #[educe(Debug(method(super::fmt_hide)))]
    pub cert: String,
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
