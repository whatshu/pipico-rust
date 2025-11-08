#[cfg(feature = "usb-serial")]
pub mod serial;

pub mod hid;

// Re-export for use in main.rs
pub use embassy_usb::Config;

// USB 配置常量
pub const USB_VID: u16 = 0x2e8a; // Raspberry Pi

// 使用不同的 PID 以避免与其他设备冲突
#[cfg(feature = "usb-serial")]
pub const USB_PID: u16 = 0x000c; // CDC + HID 复合设备

#[cfg(not(feature = "usb-serial"))]
pub const USB_PID: u16 = 0x000a; // HID 键盘

pub const USB_MANUFACTURER: &str = "RP1 Embassy";

#[cfg(feature = "usb-serial")]
pub const USB_PRODUCT: &str = "RP1 Serial + Keyboard";

#[cfg(not(feature = "usb-serial"))]
pub const USB_PRODUCT: &str = "RP1 HID Keyboard";

pub const USB_SERIAL_NUMBER: &str = "123456789ABC";

/// 创建 USB 配置
pub fn create_usb_config() -> Config<'static> {
    let mut config = Config::new(USB_VID, USB_PID);
    config.manufacturer = Some(USB_MANUFACTURER);
    config.product = Some(USB_PRODUCT);
    config.serial_number = Some(USB_SERIAL_NUMBER);
    
    // Windows 兼容性设置
    config.max_power = 100; // 200mA (100 * 2mA)
    config.max_packet_size_0 = 64; // 控制端点最大包大小
    
    // 重要：使用 USB 2.0 而不是 USB 2.1
    // Windows 对 USB 2.1 设备需要 Microsoft OS 2.0 描述符
    // 使用 USB 2.0 可以避免这个问题
    // embassy-usb 默认使用 USB 2.0，这里不需要额外设置
    
    #[cfg(feature = "usb-serial")]
    {
        // 复合设备配置 (CDC + HID)
        // Windows 兼容性的关键配置！
        
        // 🔥 尝试方案1: 使用 CDC 作为主类（更兼容某些 Windows 版本）
        // 这种配置在一些 Windows 系统上更可靠
        config.device_class = 0x02;    // CDC (Communications Device Class)
        config.device_sub_class = 0x00;
        config.device_protocol = 0x00;
        
        // 方案2: 使用 IAD 标准复合设备（需要 Windows Vista SP2+）
        // 如果方案1仍然失败，取消注释下面的代码并注释掉上面的
        // config.device_class = 0xEF;    // Miscellaneous Device
        // config.device_sub_class = 0x02; // Common Class
        // config.device_protocol = 0x01;  // Interface Association Descriptor
        
        // 注意：embassy-usb 会自动为 CDC-ACM 添加 IAD
        // CDC-ACM 必须先创建（占用接口 0 和 1）
        // HID 必须后创建（占用接口 2）
        // 接口顺序很重要！Windows 对此非常敏感
    }
    
    #[cfg(not(feature = "usb-serial"))]
    {
        // 仅 HID 设备
        config.device_class = 0x00;    // 由接口定义
        config.device_sub_class = 0x00;
        config.device_protocol = 0x00;
    }
    
    config
}


