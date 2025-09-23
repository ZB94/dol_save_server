use std::path::PathBuf;

use path_absolutize::Absolutize;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    /// 游戏名称
    pub name: String,
    /// 游戏根目录
    pub root: PathBuf,
    /// 访问"/"时的默认文件名
    #[serde(default)]
    pub index: String,
    /// 存档保存目录
    pub save_dir: PathBuf,
    /// 启动时跳过初始化模组流程
    pub init_mod: bool,
}

impl Game {
    #[instrument]
    pub fn init(&mut self) {
        // 将存档目录转为绝对路径
        self.save_dir = self
            .save_dir
            .absolutize()
            .inspect_err(|error| error!(%error, "将存档目录转为绝对路径失败"))
            .expect("将存档目录转为绝对路径失败")
            .to_path_buf();

        // 获取有效index
        let root = self.root.as_path();
        let index = if self.index.is_empty() {
            None
        } else {
            Some(root.join(&self.index))
        };

        let index = if let Some(index) = index
            && index.exists()
        {
            index
        } else {
            let pattern = root.join("*.html");
            debug!("pattern: {pattern:?}");
            glob::glob(&format!("{}", pattern.display()))
                .inspect_err(|error| error!(%error, "遍历游戏目录失败"))
                .expect("遍历游戏目录失败")
                .find_map(Result::<_, _>::ok)
                .expect("未在游戏根目录中未找到HTML文件")
        };
        info!("index: {index:?}");
        self.index = index.display().to_string();

        // 初始化模组
        if self.init_mod {
            crate::init_mod(&self.root).expect("初始化存档模组失败");
        }
    }
}
