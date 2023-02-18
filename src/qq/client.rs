//! 客户端登录

use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::elr;

use super::{CfgErr, CfgKind, OprKind, FILE_CLIENTS};

/// 协议
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub(crate) enum Protocol {
    MacOS,
    IPAD,
    QiDian,
    AndroidPhone,
    AndroidWatch,
}
impl Protocol {
    pub fn as_rq_protocol(&self) -> ricq::version::Protocol {
        use ricq::version::Protocol;
        match self {
            Self::AndroidPhone => Protocol::AndroidPhone,
            Self::AndroidWatch => Protocol::AndroidWatch,
            Self::QiDian => Protocol::QiDian,
            Self::MacOS => Protocol::MacOS,
            Self::IPAD => Protocol::IPad,
        }
    }
    pub fn as_version(&self) -> ricq::version::Version {
        ricq::version::get_version(self.as_rq_protocol())
    }
}

/// 登录配置实体
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginConf {
    pub protocol: Protocol,
    pub auto_login: bool,
    pub id: i64,
}
impl LoginConf {
    /// 默认值
    pub fn default_by_id(id: i64) -> Self {
        LoginConf {
            protocol: Protocol::MacOS,
            auto_login: true,
            id,
        }
    }
}

type Bots = Vec<LoginConf>;
type BotMap = HashMap<i64, LoginConf>;
type IOError = std::io::Error;

/// 异常
macro_rules! cfg_err {
    ($opr:expr, $opt:expr) => {
        Err(CfgErr(CfgKind::Client, $opr, $opt))
    };
}

/// 根据配置目录罗列出 bot 账号集合
fn list_id_from_dir<P: AsRef<Path>>(root: P) -> Vec<i64> {
    let mut ls_id = Vec::with_capacity(8);
    let tree_dir = elr!(std::fs::read_dir(root) ;; return vec![]);
    // 遍历配置目录，收集账号
    for tmp in tree_dir {
        let dir = elr!(tmp ;; continue);
        if dir.path().is_file() {
            continue;
        }
        let name = elr!(dir.file_name().into_string() ;; continue);
        let id = elr!(name.parse::<i64>() ;; continue);
        ls_id.push(id);
    }
    ls_id
}

/// 读取配置，组装为map<id, info>
async fn read_file<P: AsRef<Path>>(root: P) -> BotMap {
    let path = root.as_ref().join(FILE_CLIENTS);
    let ls: Bots = if path.exists() {
        let file = elr!(tokio::fs::File::open(path).await ;; return HashMap::new());
        let file = file.into_std().await;
        elr!(serde_json::from_reader(&file) ;; vec![])
    } else {
        vec![]
    };
    let mut map = HashMap::with_capacity(ls.len());
    for b in ls {
        map.insert(b.id, b);
    }
    map
}

/// 更新配置信息
/// TODO 处理异常，写前备份
async fn update_conf<P: AsRef<Path>>(clients: &Bots, path: P) -> Result<(), CfgErr> {
    let json = match serde_json::to_string_pretty(clients) {
        Err(e) => cfg_err!(OprKind::Serialization, Some(format!("{:?}", e)))?,
        Ok(s) => s,
    };
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .await;
    let mut file = match file {
        Err(e) => cfg_err!(OprKind::Write, Some(format!("{:?}", e)))?,
        Ok(f) => f,
    };
    if let Err(e) = file.write_all(json.as_bytes()).await {
        return cfg_err!(OprKind::Write, Some(format!("{:?}", e)));
    }
    Ok(())
}

/// 收集配置目录中的bot
/// TODO 处理异常
async fn collect_bot<P: AsRef<Path>>(root: P) -> Result<Bots, CfgErr> {
    let ids = list_id_from_dir(&root);
    if ids.is_empty() {
        return cfg_err!(OprKind::NotFound, None);
    }
    // 获取配置文件
    let mut map = read_file(&root).await;
    // 筛选有配置目录的bot
    let mut ls_cli = Vec::with_capacity(8);
    let mut count_new = 0;
    for id in ids {
        if let Some(cfg) = map.remove(&id) {
            ls_cli.push(cfg);
        } else {
            count_new += 1;
            ls_cli.push(LoginConf::default_by_id(id));
        }
    }
    for (_, cfg) in map {
        ls_cli.push(cfg);
    }
    // 如果有新增，执行更新
    if count_new > 0 {
        tracing::info!("新增{}个bot配置", count_new);
        update_conf(&ls_cli, root.as_ref().join(FILE_CLIENTS)).await?;
    }
    Ok(ls_cli)
}

/// 构造客户端连接
/// # argument
/// - `dir` 配置目录
pub(crate) async fn get_clients<P: AsRef<Path>>(dir: P) -> Result<(), CfgErr> {
    // let dir = dir.as_ref();
    // let path = dir.join(FILE_CLIENTS);
    // let ls_conf = collect_bot(path).await?;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qq::DIR_CFG;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn ts_print_cfg() {
        match aw!(collect_bot(Path::new(DIR_CFG))) {
            Err(e) => println!("{:#?}", e),
            Ok(ls) => for cfg in ls {
                println!("{:?}", cfg)
            }
        }
    }
}// mod tests
