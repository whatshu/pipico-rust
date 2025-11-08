//! 异步日志系统模块
//! 
//! 提供统一的串口日志输出功能，包含：
//! - 异步日志宏（log_info!, log_debug!, log_warn!, log_error!）
//! - 同步日志宏（log_info_sync!, log_debug_sync!, log_warn_sync!, log_error_sync!）
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
/// 容量增加到 16 以提高吞吐量
pub static CHANNEL: Channel<CriticalSectionRawMutex, String<256>, 16> = Channel::new();

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

/// INFO 级别日志宏（异步版本）
/// 
/// 在 async 上下文中使用，会等待直到消息发送成功
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

/// DEBUG 级别日志宏（异步版本）
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

/// WARN 级别日志宏（异步版本）
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

/// ERROR 级别日志宏（异步版本）
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

// ============================================================================
// 同步版本的日志宏（非阻塞）
// 这些宏可以在非 async 上下文中使用
// ============================================================================

/// INFO 级别日志宏（同步版本，非阻塞）
/// 
/// 可以在非 async 上下文中使用，如果 Channel 满了会丢弃消息
/// 
/// # 示例
/// ```
/// log_info_sync!("Core0", "System initialized");
/// ```
#[macro_export]
macro_rules! log_info_sync {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [INFO ] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        let _ = $crate::logger::CHANNEL.try_send(s);
    }};
}

/// DEBUG 级别日志宏（同步版本，非阻塞）
#[macro_export]
macro_rules! log_debug_sync {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [DEBUG] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        let _ = $crate::logger::CHANNEL.try_send(s);
    }};
}

/// WARN 级别日志宏（同步版本，非阻塞）
#[macro_export]
macro_rules! log_warn_sync {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [WARN ] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        let _ = $crate::logger::CHANNEL.try_send(s);
    }};
}

/// ERROR 级别日志宏（同步版本，非阻塞）
#[macro_export]
macro_rules! log_error_sync {
    ($core:expr, $($arg:tt)*) => {{
        use core::fmt::Write as _;
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [ERROR] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        let _ = $crate::logger::CHANNEL.try_send(s);
    }};
}

