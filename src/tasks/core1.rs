//! Core 1 任务

use embassy_executor::task;
use embassy_time::{Duration, Timer};

use crate::config::task::{CORE1_CHECKPOINT, CORE1_INTERVAL_MS};
use crate::{log_debug, log_info};

/// Core 1 主任务
/// 
/// 每 1.5 秒输出一次心跳信息，并在达到检查点时输出 DEBUG 日志
#[task]
pub async fn core1_task() {
    let mut counter = 0u32;
    
    log_info!("Core1", "Task started");
    
    loop {
        log_info!("Core1", "Heartbeat, count={}", counter);
        
        if counter % CORE1_CHECKPOINT == 0 && counter > 0 {
            log_debug!("Core1", "Checkpoint: {}", counter);
        }
        
        counter += 1;
        Timer::after(Duration::from_millis(CORE1_INTERVAL_MS)).await;
    }
}

