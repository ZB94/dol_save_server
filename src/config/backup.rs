use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Backup {
    pub enable: bool,
    /// 备份周期 默认1小时
    #[serde(
        deserialize_with = "duration_str::deserialize_duration",
        default = "default_period"
    )]
    pub period: Duration,
    pub backup_on_start: bool,
    #[serde(flatten)]
    pub method: BackupMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackupMethod {
    /// 备份到指定目录
    Fs { dir: PathBuf },
    /// 邮件备份
    Mail {
        smtp_host: String,
        smtp_port: u16,
        username: String,
        password: String,
        sender: String,
        receiver: Vec<String>,
    },
}

impl Default for BackupMethod {
    fn default() -> Self {
        Self::Fs {
            dir: Path::new("backup").to_path_buf(),
        }
    }
}

fn default_period() -> Duration {
    Duration::from_secs(3600)
}
