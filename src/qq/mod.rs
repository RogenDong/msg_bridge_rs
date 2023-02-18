//! qq机器人登录、事件处理
#![allow(unused)]

mod device;
mod token;

use std::path::Path;

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
    LoginError,
    GetConfigError(CfgErr),
}

/// 登录
pub(crate) async fn login() -> Result<(), LoginError> {
    let conf_dir = Path::new(DIR_CFG);
    let device = match device::get_device(conf_dir).await {
        Ok(d) => d,
        Err(de) => return Err(LoginError::GetConfigError(de)),
    };
    let token = match token::get_token(conf_dir).await {
        Ok(t) => t,
        Err(te) => return Err(LoginError::GetConfigError(te)),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn ts_get_token() {
        let mut pp = PathBuf::new();
        pp.push(DIR_CFG);
        pp.push("368894523");
        println!("{:#?}", aw!(token::get_token(&pp)));
    }

    #[test]
    fn ts_get_device() {
        let mut pp = PathBuf::new();
        pp.push(DIR_CFG);
        pp.push("368894523");
        println!("{:#?}", aw!(device::get_device(&pp)));
    }
}// mod tests
