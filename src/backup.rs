use std::{io::Cursor, path::PathBuf, time::Duration};

use chrono::Local;

use crate::{Cfg, config::backup::BackupMethod};

pub fn get_saves(save_dir: &str, period: Duration, default_mod: bool) -> Option<Vec<PathBuf>> {
    let pattern = format!("{save_dir}/**/*.save");
    let paths = match glob::glob(&pattern) {
        Ok(p) => p,
        Err(error) => {
            warn!(%error, "搜索存档目录失败");
            return None;
        }
    };

    let mut _mod = default_mod;

    let files = paths
        .into_iter()
        .filter_map(|p| {
            if let Ok(path) = p.inspect_err(|error| warn!(%error, "遍历存档目录失败")) {
                if !_mod
                    && let Ok(mt) = path
                        .metadata()
                        .inspect_err(|error| warn!(%error, ?path, "获取文件信息失败"))
                        .and_then(|mt| {
                            mt.modified()
                                .inspect_err(|error| warn!(%error, ?path, "获取文件修改时间失败"))
                        })
                {
                    match mt.elapsed() {
                        Ok(mt) => {
                            if mt <= period {
                                _mod = true;
                            }
                        }
                        Err(error) => warn!(%error, ?path, ?mt, "计算修改时间失败"),
                    }
                }

                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    _mod.then_some(files)
}

pub fn to_zip(files: Vec<PathBuf>, save_dir: &str) -> Option<Vec<u8>> {
    let mut zip = mtzip::ZipArchive::new();
    for file in files {
        trace!(?file, save_dir, "file path");
        let path = file
            .strip_prefix(save_dir)
            .unwrap()
            .to_string_lossy()
            .to_string();
        trace!(?path, "zip path");
        zip.add_file_from_fs(file, path).done();
    }

    let mut data = Vec::<u8>::new();
    let mut cursor = Cursor::new(&mut data);
    if let Err(error) = zip.write(&mut cursor) {
        error!(%error, "压缩存档文件失败");
        None
    } else {
        Some(data)
    }
}

pub async fn backup(cfg: Cfg, default_mod: bool) {
    let save_dir = cfg.game.save_dir.to_string_lossy();
    let Some(files) = get_saves(&save_dir, cfg.backup.period, default_mod) else {
        info!("存档文件没有修改, 跳过本次备份");
        return;
    };

    if files.is_empty() {
        info!("当前无存档文件, 跳过本次备份");
        return;
    }

    let Some(data) = to_zip(files, &save_dir) else {
        return;
    };

    let now = Local::now();
    match &cfg.backup.method {
        BackupMethod::Fs { dir } => {
            if let Err(error) = std::fs::create_dir_all(dir) {
                error!(%error, "创建存档备份文件夹失败");
                return;
            }

            let name = now.format("%Y%m%d%H%M%S.zip").to_string();
            let out = dir.join(name);
            if let Err(error) = std::fs::write(out, data) {
                error!(%error, "备份存档文件失败");
            } else {
                info!("已保存备份文件");
            }
        }
        BackupMethod::Mail {
            smtp_host,
            smtp_port,
            username,
            password,
            sender,
            receiver,
        } => {
            let msg = mail_send::mail_builder::MessageBuilder::new()
                .from(sender.clone())
                .to(receiver.clone())
                .subject(format!("{} - {}", &cfg.backup.title, now.format("%F %T")))
                .attachment("application/zip", "backup.zip", data);

            let mut client = match mail_send::SmtpClientBuilder::new(smtp_host.clone(), *smtp_port)
                .credentials((username.clone(), password.clone()))
                .connect()
                .await
            {
                Ok(c) => c,
                Err(error) => {
                    error!(%error, "连接发件服务器失败");
                    return;
                }
            };

            if let Err(error) = client.send(msg).await {
                error!(%error, "发送邮件失败");
            } else {
                info!("已发送备份邮件");
            }
        }
    }
}
