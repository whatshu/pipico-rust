// 仅在启用 usb-serial feature 时编译此模块
#![cfg(feature = "usb-serial")]

use defmt::*;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::Driver;
use embassy_usb::Builder;

const CDC_MAX_PACKET_SIZE: usize = 64;

/// 创建 USB CDC-ACM 类
pub fn create_cdc_acm<'d, D: Driver<'d>>(
    builder: &mut Builder<'d, D>,
    state: &'d mut State<'d>,
) -> CdcAcmClass<'d, D> {
    CdcAcmClass::new(builder, state, CDC_MAX_PACKET_SIZE as u16)
}

/// 运行 CDC-ACM (简单回显)
pub async fn run_cdc_acm<'d, D: Driver<'d>>(
    mut class: CdcAcmClass<'d, D>,
) {
    info!("USB Serial task started");
    crate::log_info_sync!("USB-Serial", "CDC-ACM task running");
    
    loop {
        crate::log_info_sync!("USB-Serial", "Waiting for connection...");
        class.wait_connection().await;
        info!("USB Serial connected");
        crate::log_info_sync!("USB-Serial", "Host connected! Echo mode active.");
        
        let mut packet_count = 0u32;
        loop {
            let mut buf = [0u8; 64];
            match class.read_packet(&mut buf).await {
                Ok(n) if n > 0 => {
                    packet_count += 1;
                    // 回显数据
                    if let Err(e) = class.write_packet(&buf[..n]).await {
                        warn!("USB Serial write error: {:?}", e);
                        crate::log_warn_sync!("USB-Serial", "Write error, disconnecting");
                        break;
                    }
                    
                    // 每 100 个包报告一次
                    if packet_count % 100 == 0 {
                        crate::log_debug_sync!("USB-Serial", "Echoed {} packets", packet_count);
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    warn!("USB Serial read error: {:?}", e);
                    crate::log_warn_sync!("USB-Serial", "Read error: {:?}", e);
                    break;
                }
            }
        }
        
        info!("USB Serial disconnected");
        crate::log_info_sync!("USB-Serial", "Host disconnected (packets: {})", packet_count);
    }
}
