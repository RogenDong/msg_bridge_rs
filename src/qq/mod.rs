//! qq机器人登录、事件处理
#![allow(unused)]

mod client;
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
    // 1. 读取登录配置clients.json，其中包含账号、是否自动登录
    // 2. 遍历账号，逐一登录，收集客户端句柄
    //      2-1. 获取device
    //      2-2. 获取token，验证账号一致
    //      2-3. 构造 ricq client；收集起来
    //      2-4. 遍历 client 集合，逐一登录
    todo!()
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
