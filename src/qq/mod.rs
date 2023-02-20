//! qq机器人登录、事件处理

mod client;
mod device;
mod token;

use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, OnceLock},
};

use ricq::LoginResponse;

use crate::elr;

/// 账户相关配置目录
const DIR_CFG: &str = "bot";
/// 文件: 登录配置
pub(super) const FILE_CLIENTS: &str = "clients.json";
/// 文件: 设备信息
pub(super) const FILE_DEVICE: &str = "device.json";
/// 文件: 二进制 token 文件
const FILE_TOKEN_BIN: &str = "token.bin";
/// 文件: JSON token
const FILE_TOKEN_JSON: &str = "token.json";

type BotMap = HashMap<i64, Arc<RicqClient>>;
static MAP_QQ: OnceLock<BotMap> = OnceLock::new();

/// 获取QQ客户端集合
pub fn get_qq_bots() -> &'static BotMap {
    let mut x = 0;
    while x < u32::MAX {
        if let Some(map) = MAP_QQ.get() {
            return map;
        }
        x += 1;
    }
    panic!("get_qq_bots 超时？？")
}

/// 配置类型
#[derive(Clone, Copy, Debug)]
pub(crate) enum CfgKind {
    Client,
    Device,
    Token,
}

/// 操作类别
#[derive(Clone, Copy, Debug)]
pub(crate) enum OprKind {
    Read,
    Write,
    NotFound,
    Serialization,
    Deserialization,
}

/// 异常：配置文件
#[derive(Clone, Debug)]
pub(crate) struct CfgErr(CfgKind, OprKind, Option<String>);

/// 异常：登录
#[derive(Clone, Debug)]
pub(crate) enum LoginError {
    GetConfigError(CfgErr),
    TokenLoginFailed,
    WrongToken,
}

type RicqClient = ricq::Client;

/// 尝试登录
async fn try_login(id: i64, cli: &Arc<RicqClient>) -> Result<(), LoginError> {
    let dir = Path::new(DIR_CFG).join(id.to_string());
    let token = match token::get_token(&dir).await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("{:#?}", e);
            return Err(LoginError::GetConfigError(e));
        }
    };
    if token.uin != id {
        tracing::error!("账号{}的 token不合法！", id);
        return Err(LoginError::WrongToken);
    }
    let resp = match cli.token_login(token).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("{:#?}", e);
            return Err(LoginError::TokenLoginFailed);
        }
    };
    if let LoginResponse::Success(_) = resp {
        ricq::ext::common::after_login(cli).await;
        // update token bin
        if let Err(e) = token::upd_token_bin(&dir, cli).await {
            tracing::error!("{:#?}", e);
        }
        return Ok(());
    }
    Err(LoginError::TokenLoginFailed)
}

/// 登录已配置账户
pub(crate) async fn login() -> Result<(), LoginError> {
    let map_cli = match client::get_clients(DIR_CFG).await {
        Ok(map) => map,
        Err(e) => {
            tracing::error!("{:#?}", e);
            return Err(LoginError::GetConfigError(e));
        }
    };
    let mut map_bot = HashMap::with_capacity(map_cli.len());
    for (id, cli) in map_cli.into_iter() {
        let arc_cli = Arc::new(cli);
        elr!(try_login(id, &arc_cli).await ;; continue);
        map_bot.insert(id, arc_cli);
    }
    MAP_QQ.get_or_init(move || map_bot);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn ts_get_client() {
        match aw!(client::get_clients(DIR_CFG)) {
            Err(e) => println!("{:#?}", e),
            Ok(v) => {
                for (id, cc) in v {
                    println!("{}: {:#?}", id, cc.account_info);
                }
            }
        } // match
    }

    #[test]
    fn ts_get_token() {
        let pp = Path::new(DIR_CFG).join("368894523");
        println!("{:#?}", aw!(token::get_token(&pp)));
    }

    #[test]
    fn ts_get_device() {
        let pp = Path::new(DIR_CFG).join("368894523");
        println!("{:#?}", aw!(device::get_device(&pp)));
    }
} // mod tests
