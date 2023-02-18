use std::path::Path;

use ricq::client::Token;

use crate::elr;

use super::{CfgErr, CfgKind, OprKind, FILE_TOKEN_BIN, FILE_TOKEN_JSON};

/// 异常
macro_rules! cfg_err {
    ($opr:expr, $opt:expr) => {
        Err(CfgErr(CfgKind::Token, $opr, $opt))
    };
}

fn get(src: &Vec<u8>, index: &mut usize, len: usize) -> Result<Vec<u8>, CfgErr> {
    if *index + len > src.len() {
        return cfg_err!(OprKind::Deserialization, None);
    }
    let res = src[*index..len].to_vec();
    *index += len;
    Ok(res)
}

fn get_i64(src: &Vec<u8>, index: &mut usize) -> Result<i64, CfgErr> {
    if *index + 8 > src.len() {
        return cfg_err!(OprKind::Deserialization, None);
    }
    let mut res = [0; 8];
    for x in 0..9 {
        res[x] = src[*index];
        *index += 1;
    }
    Ok(i64::from_be_bytes(res))
}

/// 从字节集合反序列化为 token
pub(crate) fn bytes_to_token(token: Vec<u8>) -> Result<Token, CfgErr> {
    // 切片指针
    let mut x = 0;
    Ok(Token {
        uin: get_i64(&token, &mut x)?,
        d2: get(&token, &mut x, 4)?,
        d2key: get(&token, &mut x, 4)?,
        tgt: get(&token, &mut x, 4)?,
        srm_token: get(&token, &mut x, 4)?,
        t133: get(&token, &mut x, 4)?,
        encrypted_a1: get(&token, &mut x, 4)?,
        out_packet_session_id: get(&token, &mut x, 4)?,
        tgtgt_key: get(&token, &mut x, 4)?,
        wt_session_ticket_key: get(&token, &mut x, 4)?,
    })
}

/// 将 token 序列化为字节集合
pub(crate) fn token_to_bytes(t: &Token) -> Vec<u8> {
    let mut buf = Vec::with_capacity(44);
    buf.extend_from_slice(&t.uin.to_be_bytes());
    buf.extend(&t.d2);
    buf.extend(&t.d2key);
    buf.extend(&t.tgt);
    buf.extend(&t.srm_token);
    buf.extend(&t.t133);
    buf.extend(&t.encrypted_a1);
    buf.extend(&t.out_packet_session_id);
    buf.extend(&t.tgtgt_key);
    buf.extend(&t.wt_session_ticket_key);
    buf
}

/// 从文件中获取 token
pub(super) async fn get_token(dir: &Path) -> Result<Token, CfgErr> {
    use tokio::fs::{read, File};
    if !dir.exists() {
        return cfg_err!(OprKind::NotFound, None);
    }
    if let Ok(bytes) = read(dir.join(FILE_TOKEN_BIN)).await {
        return Ok(bytes_to_token(bytes)?);
    }
    let json = elr!(File::open(dir.join(FILE_TOKEN_JSON)).await ;; cfg_err!(OprKind::Read, None)?);
    let json = json.into_std().await;
    let token = elr!(serde_json::from_reader(&json) ;; cfg_err!(OprKind::Deserialization, None)?);
    Ok(token)
}
