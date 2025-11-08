#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::PIN_25;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR1: StaticCell<embassy_executor::Executor> = StaticCell::new();

#[embassy_executor::task]
async fn blink_task(pin: PIN_25) {
    let mut led = Output::new(pin, Level::Low);

    loop {
        info!("Core 0: LED ON");
        led.set_high();
        Timer::after(Duration::from_millis(250)).await;

        info!("Core 0: LED OFF");
        led.set_low();
        Timer::after(Duration::from_millis(250)).await;
    }
}

#[embassy_executor::task]
async fn core1_task() {
    let p = unsafe { embassy_rp::Peripherals::steal() };
    let mut led = Output::new(p.PIN_0, Level::Low);

    loop {
        info!("Core 1: LED ON");
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        info!("Core 1: LED OFF");
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Hello from core 0!");

    spawner.spawn(blink_task(p.PIN_25)).unwrap();

    #[allow(static_mut_refs)]
    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(embassy_executor::Executor::new());
        info!("Hello from core 1!");
        executor1.run(|spawner| {
            spawner.spawn(core1_task()).unwrap();
        });
    });
}

