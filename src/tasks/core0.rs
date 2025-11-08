//! Core 0 任务

use embassy_executor::task;
use embassy_time::{Duration, Timer};

use crate::config::task::{CORE0_INTERVAL_MS, CORE0_MILESTONE};
use crate::{log_debug, log_info};

/// Core 0 主任务
/// 
/// 每秒输出一次心跳信息，并在达到里程碑时输出 DEBUG 日志
#[task]
pub async fn core0_task() {
    let mut counter = 0u32;
    
    log_info!("Core0", "Task started");
    
    loop {
        log_info!("Core0", "Heartbeat, count={}", counter);
        
        if counter % CORE0_MILESTONE == 0 && counter > 0 {
            log_debug!("Core0", "Milestone reached: {}", counter);
        }
        
        counter += 1;
        Timer::after(Duration::from_millis(CORE0_INTERVAL_MS)).await;
    }
}

