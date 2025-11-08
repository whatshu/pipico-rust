#![no_std]
#![no_main]

mod banner;
mod config;
mod logger;
mod tasks;
mod usb;

use defmt::*;
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

    info!("Hardware initialized");

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

    info!("UART0 configured");

    // 首先启动 UART 日志任务（这样后续的日志才能输出）
    spawner.spawn(uart_task(uart)).unwrap();

    // 等待 UART 任务启动
    embassy_time::Timer::after_millis(10).await;

    // 现在可以发送启动横幅了
    send_banner().await;

    log_info!("Main", "System initialization starting...");

    // 初始化 USB 复合设备
    log_info!("Main", "Initializing USB composite device...");
    spawner.spawn(usb_composite_task(p.USB)).unwrap();
    
    log_info!("Main", "USB composite device task spawned");

    // 启动 Core 0 任务
    log_info!("Main", "Spawning Core 0 task");
    spawner.spawn(core0_task()).unwrap();

    // 启动 Core 1
    log_info!("Main", "Spawning Core 1");
    #[allow(static_mut_refs)]
    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(embassy_executor::Executor::new());
        // 注意：这里的 info! 是 defmt，不是异步日志
        info!("Core 1 executor initialized");
        executor1.run(|spawner| {
            spawner.spawn(core1_task()).unwrap();
        });
    });

    // 等待一小段时间让所有任务启动
    embassy_time::Timer::after_millis(100).await;

    log_info!("Main", "====================================");
    log_info!("Main", "System startup complete!");
    log_info!("Main", "- UART0: GPIO0(TX) / GPIO1(RX)");
    
    #[cfg(feature = "usb-serial")]
    log_info!("Main", "- USB: HID Keyboard + CDC-ACM Serial");
    
    #[cfg(not(feature = "usb-serial"))]
    log_info!("Main", "- USB: HID Keyboard only");
    
    log_info!("Main", "- Dual Core: Core0 + Core1 running");
    log_info!("Main", "====================================");
}

/// USB 复合设备任务
#[embassy_executor::task]
async fn usb_composite_task(usb_periph: USB) {
    // 等待一下让日志系统稳定
    embassy_time::Timer::after_millis(50).await;
    
    #[cfg(feature = "usb-serial")]
    log_info_sync!("USB", "USB composite device task started (HID + CDC-ACM)");
    
    #[cfg(not(feature = "usb-serial"))]
    log_info_sync!("USB", "USB device task started (HID only)");
    
    info!("USB task: Initializing driver");
    
    // 初始化 USB 驱动
    let driver = embassy_rp::usb::Driver::new(usb_periph, Irqs);
    log_info_sync!("USB", "USB driver created");
    
    // 创建配置
    let config = usb::create_usb_config();
    log_info_sync!("USB", "USB config created (VID:0x{:04X} PID:0x{:04X})", 
                   usb::USB_VID, usb::USB_PID);
    
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
    log_info_sync!("USB", "USB builder created");
    
    // 创建 USB 串口 (CDC-ACM) - 仅在启用 feature 时
    #[cfg(feature = "usb-serial")]
    let cdc_acm = {
        static CDC_STATE: StaticCell<embassy_usb::class::cdc_acm::State> = StaticCell::new();
        let cdc_state = CDC_STATE.init(embassy_usb::class::cdc_acm::State::new());
        let cdc = usb::serial::create_cdc_acm(&mut builder, cdc_state);
        log_info_sync!("USB", "CDC-ACM serial port created");
        cdc
    };
    
    // 创建 USB HID 键盘
    static HID_HANDLER: StaticCell<usb::hid::HidRequestHandler> = StaticCell::new();
    static KEYBOARD_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();
    
    let hid_handler = HID_HANDLER.init(usb::hid::HidRequestHandler {});
    let keyboard_state = KEYBOARD_STATE.init(embassy_usb::class::hid::State::new());
    
    let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);
    log_info_sync!("USB", "HID keyboard created");
    
    // 构建 USB 设备
    let mut usb_device = builder.build();
    
    #[cfg(feature = "usb-serial")]
    log_info_sync!("USB", "USB composite device built successfully (HID + CDC-ACM)");
    
    #[cfg(not(feature = "usb-serial"))]
    log_info_sync!("USB", "USB device built successfully (HID only)");
    
    log_info_sync!("USB", "Waiting for USB enumeration...");
    info!("USB: Device ready, starting main loop");
    
    // 运行 USB 设备和各个类
    let usb_fut = usb_device.run();
    let keyboard_fut = usb::hid::run_keyboard(keyboard);
    
    #[cfg(feature = "usb-serial")]
    {
        let cdc_fut = usb::serial::run_cdc_acm(cdc_acm);
        log_info_sync!("USB", "All USB tasks starting (HID + CDC-ACM)...");
        embassy_futures::join::join3(usb_fut, cdc_fut, keyboard_fut).await;
    }
    
    #[cfg(not(feature = "usb-serial"))]
    {
        log_info_sync!("USB", "All USB tasks starting (HID only)...");
        embassy_futures::join::join(usb_fut, keyboard_fut).await;
    }
    
    log_error_sync!("USB", "USB tasks exited unexpectedly!");
}

