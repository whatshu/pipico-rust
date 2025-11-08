use defmt::*;

// 注意：embassy-usb 0.3.0 目前不支持 MSC (Mass Storage Class)
// 这是一个占位符模块，用于将来实现

/// 存储处理器占位符
pub struct StorageHandler {
    // 占位符
}

impl StorageHandler {
    pub fn new() -> Self {
        warn!("USB Mass Storage is not yet implemented in embassy-usb 0.3.0");
        warn!("This is a placeholder for future implementation");
        Self {}
    }
}

// 注释掉创建和任务函数，因为目前不支持
// 如果未来 embassy-usb 支持 MSC，可以取消注释并实现

/*
/// 创建 USB MSC 类 (未实现)
pub fn create_msc<'d, D: Driver<'d>>(
    builder: &mut Builder<'d, D>,
    handler: &'d mut StorageHandler,
) -> ... {
    unimplemented!("MSC not supported in embassy-usb 0.3.0")
}

/// USB MSC 任务 (未实现)
#[embassy_executor::task]
pub async fn usb_msc_task(...) {
    info!("USB MSC task started");
    // 未实现
}
*/
