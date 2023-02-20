//! 设备信息

use std::path::Path;

use ricq::Device;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt};

use crate::elr;

use super::{CfgErr, CfgKind, OprKind, FILE_DEVICE};

/// 异常
macro_rules! cfg_err {
    ($opr:expr, $opt:expr) => {
        Err(CfgErr(CfgKind::Device, $opr, $opt))
    };
}

/// 新建设备信息
async fn new_device<P: AsRef<Path>>(path: P) -> Result<Device, CfgErr> {
    let mut file = elr!(File::create(&path).await ;; cfg_err!(OprKind::Write, None)?);
    let new = Device::random();
    let s = elr!(serde_json::to_string(&new) ;; cfg_err!(OprKind::Serialization, None)?);
    if let Err(e) = file.write_all(s.as_bytes()).await {
        return cfg_err!(OprKind::Write, Some(format!("{:?}", e)));
    }
    Ok(new)
}

/// 读取或新建设备信息
pub(super) async fn get_device(dir: &Path) -> Result<Device, CfgErr> {
    if !dir.exists() {
        return cfg_err!(OprKind::NotFound, None);
    }
    let path = dir.join(FILE_DEVICE);
    if !path.exists() || !path.is_file() {
        return new_device(path).await;
    }
    // 读取
    let file = elr!(File::open(&path).await ;; cfg_err!(OprKind::Read, None)?);
    let file = file.into_std().await;
    let device = elr!(serde_json::from_reader(&file) ;; cfg_err!(OprKind::Deserialization, None)?);
    Ok(device)
}
