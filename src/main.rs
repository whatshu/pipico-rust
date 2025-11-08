#![no_std]
#![no_main]

mod banner;
mod config;
mod logger;
mod tasks;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::UART0;
use embassy_rp::uart::{Config, InterruptHandler, Uart};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use banner::send_banner;
use config::{uart, CORE1_STACK_SIZE};
use logger::uart_task;
use tasks::{core0_task, core1_task};

bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<UART0>;
});

static mut CORE1_STACK: Stack<CORE1_STACK_SIZE> = Stack::new();
static EXECUTOR1: StaticCell<embassy_executor::Executor> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // 配置 UART0
    let mut config = Config::default();
    config.baudrate = uart::BAUD_RATE;
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
    send_banner().await;

    info!("System initialization complete");

    // 启动 UART 日志任务
    spawner.spawn(uart_task(uart)).unwrap();

    // 启动 Core 0 任务
    log_info!("Core0", "Initializing Core 0 executor");
    spawner.spawn(core0_task()).unwrap();

    // 启动 Core 1
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

