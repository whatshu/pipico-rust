#![no_std]
#![no_main]

mod banner;
mod config;
mod logger;
mod tasks;
mod usb;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{UART0, USB};
use embassy_rp::uart::{Config, InterruptHandler as UartInterruptHandler, Uart};
use embassy_rp::usb::InterruptHandler as UsbInterruptHandler;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use banner::send_banner;
use config::{uart, CORE1_STACK_SIZE};
use logger::uart_task;
use tasks::{core0_task, core1_task};

bind_interrupts!(struct Irqs {
    UART0_IRQ => UartInterruptHandler<UART0>;
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

static mut CORE1_STACK: Stack<CORE1_STACK_SIZE> = Stack::new();
static EXECUTOR1: StaticCell<embassy_executor::Executor> = StaticCell::new();

// 注意：之前的 USB_DRIVER, USB_DEVICE, HID_REQUEST_HANDLER 等静态变量已移到 usb_composite_task 中
// 避免全局静态变量，使用函数内部的 StaticCell 更安全

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 初始化硬件
    let p = embassy_rp::init(Default::default());

    // 配置 UART0 - 用于日志输出
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

    // 首先启动 UART 日志任务（这样后续的日志才能输出）
    spawner.spawn(uart_task(uart)).unwrap();

    // 等待 UART 任务启动
    embassy_time::Timer::after_millis(10).await;

    // 现在可以发送启动横幅了
    send_banner().await;

    log_info!("Main", "System initialization starting...");

    // 初始化 USB HID 键盘
    log_info!("Main", "Initializing USB HID keyboard...");
    spawner.spawn(usb_composite_task(p.USB)).unwrap();
    
    log_info!("Main", "USB HID keyboard task spawned");

    // 启动 Core 0 任务
    log_info!("Main", "Spawning Core 0 task");
    spawner.spawn(core0_task()).unwrap();

    // 启动 Core 1
    log_info!("Main", "Spawning Core 1");
    #[allow(static_mut_refs)]
    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(embassy_executor::Executor::new());
        executor1.run(|spawner| {
            spawner.spawn(core1_task()).unwrap();
        });
    });

    // 等待一小段时间让所有任务启动
    embassy_time::Timer::after_millis(100).await;

    log_info!("Main", "====================================");
    log_info!("Main", "System startup complete!");
    log_info!("Main", "- UART0: GPIO0(TX) / GPIO1(RX)");
    log_info!("Main", "- USB: HID Keyboard");
    log_info!("Main", "- Dual Core: Core0 + Core1 running");
    log_info!("Main", "====================================");
}

/// USB HID 键盘任务
#[embassy_executor::task]
async fn usb_composite_task(usb_periph: USB) {
    // 等待日志系统稳定
    embassy_time::Timer::after_millis(50).await;
    
    log_info_sync!("USB", "Initializing USB HID keyboard...");
    
    // 初始化 USB 驱动
    let driver = embassy_rp::usb::Driver::new(usb_periph, Irqs);
    
    // 创建配置
    let config = usb::create_usb_config();
    
    // 创建构建器
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CTRL_BUF: StaticCell<[u8; 128]> = StaticCell::new();
    
    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        MSOS_DESC.init([0; 256]),
        CTRL_BUF.init([0; 128]),
    );
    
    // 创建 USB HID 键盘
    static HID_HANDLER: StaticCell<usb::hid::HidRequestHandler> = StaticCell::new();
    static KEYBOARD_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();
    
    let hid_handler = HID_HANDLER.init(usb::hid::HidRequestHandler {});
    let keyboard_state = KEYBOARD_STATE.init(embassy_usb::class::hid::State::new());
    
    let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);
    
    // 构建 USB 设备
    let mut usb_device = builder.build();
    
    log_info_sync!("USB", "USB HID device ready (VID:0x{:04X} PID:0x{:04X})", 
                   usb::USB_VID, usb::USB_PID);
    
    // 运行 USB 设备和 HID 键盘
    let usb_fut = usb_device.run();
    let keyboard_fut = usb::hid::run_keyboard(keyboard);
    
    embassy_futures::join::join(usb_fut, keyboard_fut).await;
    
    log_error_sync!("USB", "USB tasks exited unexpectedly!");
}

