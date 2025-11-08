#![no_std]
#![no_main]

use core::fmt::Write as FmtWrite;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::UART0;
use embassy_rp::uart::{Config, InterruptHandler, Uart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use heapless::String;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<UART0>;
});

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR1: StaticCell<embassy_executor::Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, String<256>, 10> = Channel::new();

// 日志宏
macro_rules! log_info {
    ($core:expr, $($arg:tt)*) => {{
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [INFO ] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        CHANNEL.send(s).await;
    }};
}

macro_rules! log_debug {
    ($core:expr, $($arg:tt)*) => {{
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [DEBUG] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        CHANNEL.send(s).await;
    }};
}

macro_rules! log_warn {
    ($core:expr, $($arg:tt)*) => {{
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [WARN ] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        CHANNEL.send(s).await;
    }};
}

macro_rules! log_error {
    ($core:expr, $($arg:tt)*) => {{
        let uptime = embassy_time::Instant::now().as_millis();
        let mut s: heapless::String<256> = heapless::String::new();
        let _ = core::write!(s, "[{:>8}ms] [{}] [ERROR] ", uptime, $core);
        let _ = core::write!(s, $($arg)*);
        let _ = core::write!(s, "\r\n");
        CHANNEL.send(s).await;
    }};
}

#[embassy_executor::task]
async fn uart_task(mut uart: Uart<'static, UART0, embassy_rp::uart::Async>) {
    loop {
        let msg = CHANNEL.receive().await;
        let _ = uart.write(msg.as_bytes()).await;
    }
}

#[embassy_executor::task]
async fn core0_task() {
    let mut counter = 0u32;
    
    log_info!("Core0", "Task started");
    
    loop {
        log_info!("Core0", "Heartbeat, count={}", counter);
        
        if counter % 10 == 0 && counter > 0 {
            log_debug!("Core0", "Milestone reached: {}", counter);
        }
        
        counter += 1;
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn core1_task() {
    let mut counter = 0u32;
    
    log_info!("Core1", "Task started");
    
    loop {
        log_info!("Core1", "Heartbeat, count={}", counter);
        
        if counter % 5 == 0 && counter > 0 {
            log_debug!("Core1", "Checkpoint: {}", counter);
        }
        
        counter += 1;
        Timer::after(Duration::from_millis(1500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // 配置 UART0: GPIO0 (TX), GPIO1 (RX)
    let mut config = Config::default();
    config.baudrate = 115200;
    let uart = Uart::new(
        p.UART0,
        p.PIN_0,
        p.PIN_1,
        Irqs,
        p.DMA_CH0,
        p.DMA_CH1,
        config,
    );

    // 发送启动横幅
    let mut s: String<256> = String::new();
    let _ = core::write!(s, "\r\n");
    let _ = core::write!(s, "=====================================\r\n");
    let _ = core::write!(s, "  RP2040 Dual Core UART Demo\r\n");
    let _ = core::write!(s, "  Embassy Async Framework\r\n");
    let _ = core::write!(s, "=====================================\r\n");
    let _ = core::write!(s, "UART0 Config:\r\n");
    let _ = core::write!(s, "  - Baud Rate: 115200\r\n");
    let _ = core::write!(s, "  - TX: GPIO0, RX: GPIO1\r\n");
    let _ = core::write!(s, "  - Data: 8N1\r\n");
    let _ = core::write!(s, "=====================================\r\n");
    let _ = core::write!(s, "Log Format:\r\n");
    let _ = core::write!(s, "  [uptime_ms] [Core] [LEVEL] message\r\n");
    let _ = core::write!(s, "=====================================\r\n\r\n");
    CHANNEL.send(s).await;

    info!("System initialization complete");

    spawner.spawn(uart_task(uart)).unwrap();
    
    log_info!("Core0", "Initializing Core 0 executor");
    spawner.spawn(core0_task()).unwrap();

    #[allow(static_mut_refs)]
    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(embassy_executor::Executor::new());
        info!("Core 1 executor initialized");
        executor1.run(|spawner| {
            spawner.spawn(core1_task()).unwrap();
        });
    });
    
    log_info!("Core0", "System startup complete");
}

