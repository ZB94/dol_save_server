use std::{net::SocketAddr, path::PathBuf};

const HTLP_TEMPLATE: &str = r#"
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"#;

#[derive(Debug, clap::Parser)]
#[command(
    author,
    version,
    help_template = HTLP_TEMPLATE
)]
pub struct Args {
    /// 游戏根目录
    #[arg(long, default_value = "./")]
    pub root: PathBuf,
    /// 访问"/"时的默认文件名
    #[arg(long, default_value = "index.html")]
    pub index: String,
    /// 服务地址
    #[arg(long, default_value = "127.0.0.1:5000")]
    pub bind: SocketAddr,
    /// 存档保存目录
    #[arg(long, default_value = "./save")]
    pub save_dir: PathBuf,
}
