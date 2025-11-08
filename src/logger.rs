//! 日志系统模块
//! 
//! 提供统一的串口日志输出功能，包含：
//! - 格式化日志宏（log_info!, log_debug!, log_warn!, log_error!）
//! - UART 任务，负责从 Channel 接收日志并输出

use embassy_executor::task;
use embassy_rp::peripherals::UART0;
use embassy_rp::uart::Uart;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::String;

/// 日志消息 Channel
/// 
/// 用于在双核间共享日志输出，避免串口访问冲突
pub static CHANNEL: Channel<CriticalSectionRawMutex, String<256>, 10> = Channel::new();

/// UART 日志输出任务
/// 
/// 从 Channel 接收日志消息并通过串口发送
#[task]
pub async fn uart_task(mut uart: Uart<'static, UART0, embassy_rp::uart::Async>) {
    loop {
        let msg = CHANNEL.receive().await;
        let _ = uart.write(msg.as_bytes()).await;
    }
}

/// INFO 级别日志宏
/// 
/// # 示例
/// ```
/// log_info!("Core0", "System initialized");
/// log_info!("Core1", "Counter: {}", counter);
/// ```
#[macro_export]
macro_rules! log_info {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [INFO ] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        $crate::logger::CHANNEL.send(s).await;
    }};
}

/// DEBUG 级别日志宏
#[macro_export]
macro_rules! log_debug {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [DEBUG] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        $crate::logger::CHANNEL.send(s).await;
    }};
}

/// WARN 级别日志宏
#[macro_export]
macro_rules! log_warn {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [WARN ] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        $crate::logger::CHANNEL.send(s).await;
    }};
}

/// ERROR 级别日志宏
#[macro_export]
macro_rules! log_error {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [ERROR] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        $crate::logger::CHANNEL.send(s).await;
    }};
}

