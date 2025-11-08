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
static CHANNEL: Channel<CriticalSectionRawMutex, String<128>, 10> = Channel::new();

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
    loop {
        let mut s: String<128> = String::new();
        let _ = core::write!(s, "[Core 0] Running... count = {}\r\n", counter);
        info!("[Core 0] Running... count = {}", counter);
        CHANNEL.send(s).await;
        counter += 1;
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn core1_task() {
    let mut counter = 0u32;
    loop {
        let mut s: String<128> = String::new();
        let _ = core::write!(s, "[Core 1] Running... count = {}\r\n", counter);
        info!("[Core 1] Running... count = {}", counter);
        CHANNEL.send(s).await;
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

    info!("UART0 initialized");
    info!("Baud rate: 115200");
    info!("TX: GPIO0, RX: GPIO1");

    // 发送初始化消息
    let mut s: String<128> = String::new();
    let _ = core::write!(s, "\r\n=== RP2040 Dual Core Demo ===\r\n");
    CHANNEL.send(s).await;
    
    let mut s: String<128> = String::new();
    let _ = core::write!(s, "UART0 Baud Rate: 115200\r\n");
    CHANNEL.send(s).await;
    
    let mut s: String<128> = String::new();
    let _ = core::write!(s, "Starting dual core tasks...\r\n\r\n");
    CHANNEL.send(s).await;

    spawner.spawn(uart_task(uart)).unwrap();
    spawner.spawn(core0_task()).unwrap();

    #[allow(static_mut_refs)]
    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(embassy_executor::Executor::new());
        info!("Core 1 started");
        executor1.run(|spawner| {
            spawner.spawn(core1_task()).unwrap();
        });
    });
}

