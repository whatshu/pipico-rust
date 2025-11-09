use defmt::*;
use embassy_time::Timer;
use embassy_usb::class::hid::{HidWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::driver::Driver;
use embassy_usb::Builder;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

/// HID 请求处理器
pub struct HidRequestHandler {}

impl RequestHandler for HidRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }
    
    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {:?}", id, data);
        OutResponse::Accepted
    }
    
    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle for {:?} to {} ms", id, dur);
    }
    
    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle for {:?}", id);
        None
    }
}

/// 创建键盘 HID
pub fn create_keyboard_hid<'d, D: Driver<'d>>(
    builder: &mut Builder<'d, D>,
    state: &'d mut State<'d>,
    request_handler: &'d mut HidRequestHandler,
) -> HidWriter<'d, D, 8> {
    HidWriter::<_, 8>::new(
        builder,
        state,
        embassy_usb::class::hid::Config {
            report_descriptor: KeyboardReport::desc(),
            request_handler: Some(request_handler),
            poll_ms: 10,
            max_packet_size: 8,
        },
    )
}

// 鼠标 HID 已删除
// 如果需要鼠标功能，请参考 embassy-usb HID 示例

/// 运行键盘 HID (演示：定期发送按键)
pub async fn run_keyboard<'d, D: Driver<'d>>(
    mut keyboard: HidWriter<'d, D, 8>,
) {
    info!("USB Keyboard task started");
    crate::log_info_sync!("USB-Keyboard", "HID Keyboard task running");
    crate::log_info_sync!("USB-Keyboard", "Will send 'H' key every 5 seconds");
    
    let mut count = 0u32;
    loop {
        Timer::after_secs(1).await;
        count += 1;
        
        // 发送 'H' 键 (HID 键码 0x0B)
        let report = [0, 0, 0x0B, 0, 0, 0, 0, 0]; // modifier, reserved, keycodes...
        if let Err(e) = keyboard.write(&report).await {
            warn!("Keyboard write error: {:?}", e);
            crate::log_error_sync!("USB-Keyboard", "Write error: {:?}", e);
            continue;
        }
        
        // 释放按键
        Timer::after_millis(50).await;
        let release = [0, 0, 0, 0, 0, 0, 0, 0];
        let _ = keyboard.write(&release).await;
        
        info!("Sent keyboard report #{}", count);
        crate::log_debug_sync!("USB-Keyboard", "Sent 'H' key (count: {})", count);
    }
}

// 鼠标功能已删除
