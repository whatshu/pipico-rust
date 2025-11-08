//! 启动横幅模块

use core::fmt::Write as FmtWrite;
use heapless::String;

use crate::config::uart::BAUD_RATE;
use crate::logger::CHANNEL;

/// 发送启动横幅到串口
pub async fn send_banner() {
    let mut s: String<256> = String::new();
    let _ = core::write!(s, "\r\n");
    let _ = core::write!(s, "=====================================\r\n");
    let _ = core::write!(s, "  RP2040 Dual Core UART Demo\r\n");
    let _ = core::write!(s, "  Embassy Async Framework\r\n");
    let _ = core::write!(s, "=====================================\r\n");
    let _ = core::write!(s, "UART0 Config:\r\n");
    let _ = core::write!(s, "  - Baud Rate: {}\r\n", BAUD_RATE);
    let _ = core::write!(s, "  - TX: GPIO0, RX: GPIO1\r\n");
    let _ = core::write!(s, "  - Data: 8N1\r\n");
    let _ = core::write!(s, "=====================================\r\n");
    let _ = core::write!(s, "Log Format:\r\n");
    let _ = core::write!(s, "  [uptime_ms] [Core] [LEVEL] message\r\n");
    let _ = core::write!(s, "=====================================\r\n\r\n");
    CHANNEL.send(s).await;
}

