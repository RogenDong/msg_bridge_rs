#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use tracing::{debug, info};

use config::*;

mod bridge;
mod bridge_cmd;
mod bridge_data;
mod bridge_dc;
mod bridge_log;
mod bridge_message_history;
mod bridge_qq;
mod cmd_adapter;
mod config;
mod logger;
mod qq;
mod utils;

pub type HttpResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    // static ref BRIDGE_MESSAGE_HISTORY: Arc<Mutex<bridge_message_history::BridgeMessageHistory>> =
    //     Arc::new(Mutex::new(bridge_message_history::BridgeMessageHistory));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 守卫日志文件。如果被drop，日志将无法写入到文件
    let _logger_guards = logger::init_logger();
    let config = Arc::new(Config::new());
    debug!("config: {:#?}", config);
    info!("config loaded");
    let bridge_service = bridge::BridgeService::new();
    let bridge_service = Arc::new(Mutex::new(bridge_service));
    let bridge_dc_client =
        bridge::BridgeService::create_client("bridge_dc_client", bridge_service.clone());
    let bridge_qq_client =
        bridge::BridgeService::create_client("bridge_qq_client", bridge_service.clone());
    let bridge_cmd_adapter =
        bridge::BridgeService::create_client("bridge_cmd_adapter", bridge_service.clone());
    // let a = Some(bridge_service.clone());
    info!("bridge ready");

    tokio::select! {
        _ = bridge_dc::start(config.clone(), bridge_dc_client) => {},
        _ = bridge_qq::start(config.clone(), bridge_qq_client) => {},
        _ = cmd_adapter::start(config.clone(), bridge_cmd_adapter) => {},
    }

    Ok(())
}

/// 2元表达式宏 elr!(Ok ;; Err)
/// # Example
/// ```
/// let tt = Ok(1);
/// let x: u8 = elr!(tt ;; return);
/// ```
#[macro_export]
macro_rules! elr {
    ($opt:expr ;; $ret:expr) => {
        match $opt {
            Ok(v) => v,
            _ => $ret,
        }
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn ts_elr() {
        let ls = [Ok(0), Err(1), Ok(2)];
        for x in 0..ls.len() {
            let v = elr!(ls[x] ;; -1);
            println!("{}:{}  ", x, v);
        }
    }

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        // let config = Config::new();
        // let mut mirai = Mirai::new(
        //     &config.miraiConfig.host,
        //     config.miraiConfig.port,
        //     &config.miraiConfig.verifyKey,
        // )
        // .bind_qq(3245538509);
        // let resp = tokio_test::block_on(mirai.verify());
        // println!("{:?}", resp);
        // let resp = tokio_test::block_on(mirai.bind());

        // println!("{:?}", resp);

        Ok(())
    }

    #[test]
    fn getConfig() {
        let config = Config::new();
        println!("config:");
        println!("{:?}", config);
    }
}

mod test_dc;
mod test_mirai;
mod test_regex;
mod test_reqwest;
