pub mod group;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendGroupMessageResponse {
    code: u32,
    msg: String,
    messageId: u64,
}

/**
 * 基础响应格式
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub code: u32,
    pub msg: String,
    pub data: T,
}
