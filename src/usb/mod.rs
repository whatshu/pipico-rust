pub mod hid;

// Re-export for use in main.rs
pub use embassy_usb::Config;

// USB 配置常量
pub const USB_VID: u16 = 0x2e8a; // Raspberry Pi
pub const USB_PID: u16 = 0x000a; // HID 键盘
pub const USB_MANUFACTURER: &str = "RP1 Embassy";
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
    
    // HID 设备配置
    config.device_class = 0x00;    // 由接口定义
    config.device_sub_class = 0x00;
    config.device_protocol = 0x00;
    
    config
}


