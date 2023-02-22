use std::path::Path;

use ricq::client::Token;

use crate::elr;

use super::{CfgErr, CfgKind, OprKind, RicqClient, FILE_TOKEN_JSON};

/// 异常
macro_rules! cfg_err {
    ($opr:expr, $opt:expr) => {
        Err(CfgErr(CfgKind::Token, $opr, $opt))
    };
}

macro_rules! exs {
    ($dir:expr) => {
        if !$dir.exists() {
            return cfg_err!(OprKind::NotFound, Some($dir.display().to_string()));
        }
    };
}

/// 从文件中获取 token
pub(super) async fn get_token(dir: &Path) -> Result<Token, CfgErr> {
    exs!(dir);
    // if let Ok(bytes) = tokio::fs::read(dir.join(FILE_TOKEN_BIN)).await {
    //     return Ok(bytes_to_token(bytes)?);
    // }
    let json = elr!(tokio::fs::File::open(dir.join(FILE_TOKEN_JSON)).await ;; cfg_err!(OprKind::Read, None)?);
    let json = json.into_std().await;
    let token = elr!(serde_json::from_reader(&json) ;; cfg_err!(OprKind::Deserialization, None)?);
    Ok(token)
}

/// 保存token
/// # arguments
/// - `dir` 保存目录
/// - `client` 客户端
pub(crate) async fn upd_token_json(dir: &Path, client: &RicqClient) -> Result<(), CfgErr> {
    use tokio::io::AsyncWriteExt;
    exs!(dir);
    let path = dir.join(FILE_TOKEN_JSON);
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .await;
    let mut file = match file {
        Err(e) => cfg_err!(OprKind::Write, Some(format!("{:?}", e)))?,
        Ok(f) => f,
    };
    let token = client.gen_token().await;
    let data = elr!(serde_json::to_vec(&token) ;; cfg_err!(OprKind::Serialization, None)?);
    if let Err(e) = file.write_all(&data).await {
        return cfg_err!(OprKind::Write, Some(format!("{:?}", e)));
    }
    Ok(())
}
