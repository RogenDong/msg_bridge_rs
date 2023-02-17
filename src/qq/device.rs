//! 设备信息

use std::path::Path;

use ricq::Device;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::{ConfigError, ConfigKind, OperationKind};

/// 文件: 设备信息
pub(super) const FILE_DEVICE: &str = "device.json";

/// 新建设备信息
async fn new_device<P: AsRef<Path>>(path: P) -> Result<Device, ConfigError> {
    let Ok(mut file) = File::create(&path).await else {
        return Err(ConfigError(ConfigKind::Device, OperationKind::Write, None));
    };
    let new = Device::random();
    let Ok(s) = serde_json::to_string_pretty(&new) else {
        return Err(ConfigError(ConfigKind::Device, OperationKind::Serialization, None));
    };
    if let Err(e) = file.write_all(s.as_bytes()).await {
        return Err(ConfigError(
            ConfigKind::Device,
            OperationKind::Write,
            Some(format!("【device 写入失败】{:?}", e)),
        ));
    }
    Ok(new)
}

/// 读取或新建设备信息
pub(super) async fn get_device(dir: &Path) -> Result<Device, ConfigError> {
    let path = dir.join(FILE_DEVICE);
    if !path.is_file() {
        return new_device(path).await;
    }
    // 读取
    let Ok(mut file) = File::open(&path).await else {
        return Err(ConfigError(ConfigKind::Device, OperationKind::Read, None));
    };
    let mut buf = String::new();
    if let Err(e) = file.read_to_string(&mut buf).await {
        return Err(ConfigError(
            ConfigKind::Device,
            OperationKind::Read,
            Some(format!("【读取 device 失败】{:?}", e))
        ));
    }
    if let Ok(device) = serde_json::from_str(&buf) {
        return Ok(device);
    }
    Err(ConfigError(ConfigKind::Device, OperationKind::Deserialization, None))
}
