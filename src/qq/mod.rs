//! qq机器人登录、事件处理
#![allow(unused)]

mod device;
mod token;

use std::path::Path;

/// 账户相关配置目录
const DIR_CONF: &str = "bot";

/// 配置类型
#[derive(Clone, Copy, Debug)]
pub(crate) enum ConfigKind {
    Client,
    Device,
    Token,
}

/// 操作类别
#[derive(Clone, Copy, Debug)]
pub(crate) enum OperationKind {
    Read,
    Write,
    Serialization,
    Deserialization,
}

/// 异常：配置文件
#[derive(Clone, Debug)]
pub(crate) struct ConfigError(ConfigKind, OperationKind, Option<String>);

/// 异常：登录
#[derive(Clone, Debug)]
pub(crate) enum LoginError {
    LoginError,
    GetConfigError(ConfigError),
}

/// 登录
pub(crate) async fn login() -> Result<(), LoginError> {
    let conf_dir = Path::new(DIR_CONF);
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
