pub mod serial;
pub mod storage;
pub mod hid;

use defmt::*;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use static_cell::StaticCell;

// Re-export for use in main.rs
pub use embassy_usb::{Builder, Config};

// USB 配置常量
pub const USB_VID: u16 = 0x2e8a; // Raspberry Pi
pub const USB_PID: u16 = 0x000a; // 自定义产品 ID
pub const USB_MANUFACTURER: &str = "RP1 Embassy";
pub const USB_PRODUCT: &str = "Composite Device";
pub const USB_SERIAL_NUMBER: &str = "123456789ABC";

// 静态缓冲区 (公开以便在 main.rs 中使用)
pub static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
pub static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
pub static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();
pub static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();

/// 创建 USB 配置
pub fn create_usb_config() -> Config<'static> {
    let mut config = Config::new(USB_VID, USB_PID);
    config.manufacturer = Some(USB_MANUFACTURER);
    config.product = Some(USB_PRODUCT);
    config.serial_number = Some(USB_SERIAL_NUMBER);
    config.max_power = 100; // 200mA (100 * 2mA) - 降低功耗以提高兼容性
    config.max_packet_size_0 = 64;
    
    // 使用标准的复合设备配置 (IAD - Interface Association Descriptor)
    // 这个配置 Windows 支持得更好
    config.device_class = 0xEF;    // Miscellaneous
    config.device_sub_class = 0x02; // Common Class
    config.device_protocol = 0x01;  // Interface Association Descriptor
    
    config
}


