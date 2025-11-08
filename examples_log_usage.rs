// 这是一个示例文件，展示如何使用改进的异步日志系统
// 注意：这不是一个完整的可运行文件，只是示例代码片段

use embassy_executor::task;
use embassy_time::{Duration, Timer};

// ============================================================================
// 示例 1: 在异步任务中使用异步日志
// ============================================================================

#[task]
async fn example_async_task() {
    // 使用异步日志宏 - 会等待直到消息发送成功
    log_info!("AsyncTask", "Task started");
    
    let mut counter = 0;
    
    loop {
        counter += 1;
        
        // INFO 级别 - 正常信息
        log_info!("AsyncTask", "Processing iteration {}", counter);
        
        // DEBUG 级别 - 详细调试信息
        if counter % 10 == 0 {
            log_debug!("AsyncTask", "Checkpoint reached at iteration {}", counter);
        }
        
        // WARN 级别 - 警告
        if counter > 50 && counter < 55 {
            log_warn!("AsyncTask", "Counter approaching threshold");
        }
        
        // ERROR 级别 - 错误
        if counter > 100 {
            log_error!("AsyncTask", "Counter exceeded maximum value!");
            break;
        }
        
        Timer::after(Duration::from_millis(100)).await;
    }
    
    log_info!("AsyncTask", "Task completed");
}

// ============================================================================
// 示例 2: 在同步函数中使用同步日志
// ============================================================================

fn example_sync_function(value: u32) {
    // 使用同步日志宏 - 非阻塞，立即返回
    log_info_sync!("SyncFn", "Function called with value: {}", value);
    
    // 执行一些同步操作
    if value > 100 {
        log_warn_sync!("SyncFn", "Value {} exceeds recommended range", value);
    }
    
    // 模拟一些计算
    let result = value * 2;
    log_debug_sync!("SyncFn", "Calculated result: {}", result);
}

// ============================================================================
// 示例 3: USB 串口回显任务（使用异步日志）
// ============================================================================

#[task]
async fn usb_serial_echo_task() {
    log_info!("USB", "USB Serial echo task starting...");
    
    // 模拟 USB 连接
    Timer::after_secs(1).await;
    log_info!("USB", "USB device connected");
    
    let mut byte_count = 0;
    
    loop {
        // 模拟接收数据
        Timer::after_millis(500).await;
        byte_count += 64;
        
        log_debug!("USB", "Received {} bytes, total: {}", 64, byte_count);
        
        if byte_count % 1000 == 0 {
            log_info!("USB", "Processed {} bytes so far", byte_count);
        }
        
        // 模拟错误情况
        if byte_count == 5000 {
            log_warn!("USB", "Buffer usage high, consider flow control");
        }
    }
}

// ============================================================================
// 示例 4: 传感器读取任务（混合使用）
// ============================================================================

#[task]
async fn sensor_read_task() {
    log_info!("Sensor", "Sensor read task initialized");
    
    loop {
        // 模拟传感器读取
        let temperature = read_sensor().await;
        
        // 正常范围
        if temperature >= 20.0 && temperature <= 30.0 {
            log_debug!("Sensor", "Temperature: {:.1}°C (normal)", temperature);
        }
        // 警告范围
        else if temperature > 30.0 && temperature < 40.0 {
            log_warn!("Sensor", "Temperature: {:.1}°C (high)", temperature);
        }
        // 危险范围
        else if temperature >= 40.0 {
            log_error!("Sensor", "Temperature: {:.1}°C (critical!)", temperature);
        }
        // 低温
        else {
            log_warn!("Sensor", "Temperature: {:.1}°C (low)", temperature);
        }
        
        Timer::after_secs(2).await;
    }
}

async fn read_sensor() -> f32 {
    // 模拟传感器读取
    25.5
}

// ============================================================================
// 示例 5: 网络任务（错误处理）
// ============================================================================

#[task]
async fn network_task() {
    log_info!("Network", "Network task started");
    
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 3;
    
    loop {
        log_debug!("Network", "Attempting connection...");
        
        match connect_to_server().await {
            Ok(_) => {
                log_info!("Network", "Connected successfully");
                retry_count = 0;
                
                // 处理数据
                if let Err(e) = process_data().await {
                    log_error!("Network", "Data processing failed: {:?}", e);
                }
            }
            Err(e) => {
                retry_count += 1;
                
                if retry_count <= MAX_RETRIES {
                    log_warn!(
                        "Network", 
                        "Connection failed (attempt {}/{}): {:?}", 
                        retry_count, MAX_RETRIES, e
                    );
                } else {
                    log_error!(
                        "Network", 
                        "Connection failed after {} retries, giving up", 
                        MAX_RETRIES
                    );
                    break;
                }
            }
        }
        
        Timer::after_secs(5).await;
    }
}

async fn connect_to_server() -> Result<(), &'static str> {
    // 模拟连接
    Ok(())
}

async fn process_data() -> Result<(), &'static str> {
    // 模拟数据处理
    Ok(())
}

// ============================================================================
// 示例 6: 中断处理器（使用同步日志）
// ============================================================================

fn gpio_interrupt_handler() {
    // 在中断上下文中使用同步日志
    // 不会阻塞，即使队列满了也会立即返回
    log_info_sync!("ISR", "GPIO interrupt triggered");
    
    // 读取引脚状态
    let pin_state = read_pin_state();
    log_debug_sync!("ISR", "Pin state: {}", pin_state);
    
    // 如果发生意外状态
    if pin_state > 10 {
        log_error_sync!("ISR", "Unexpected pin state: {}", pin_state);
    }
}

fn read_pin_state() -> u32 {
    // 模拟读取引脚
    5
}

// ============================================================================
// 示例 7: 多核日志（Core 0 和 Core 1）
// ============================================================================

#[task]
async fn core0_main_task() {
    log_info!("Core0", "Core 0 task starting");
    
    let mut iteration = 0;
    loop {
        iteration += 1;
        log_debug!("Core0", "Core 0 iteration: {}", iteration);
        Timer::after_millis(500).await;
    }
}

#[task]
async fn core1_main_task() {
    log_info!("Core1", "Core 1 task starting");
    
    let mut iteration = 0;
    loop {
        iteration += 1;
        log_debug!("Core1", "Core 1 iteration: {}", iteration);
        Timer::after_millis(750).await;
    }
}

// ============================================================================
// 示例 8: 系统监控任务
// ============================================================================

#[task]
async fn system_monitor_task() {
    log_info!("Monitor", "System monitor started");
    
    loop {
        // 检查系统状态
        let free_memory = get_free_memory();
        let cpu_usage = get_cpu_usage();
        
        log_info!(
            "Monitor", 
            "Status: Free Memory: {} KB, CPU: {}%", 
            free_memory, cpu_usage
        );
        
        // 如果资源紧张
        if free_memory < 10 {
            log_warn!("Monitor", "Low memory warning: {} KB remaining", free_memory);
        }
        
        if cpu_usage > 80 {
            log_warn!("Monitor", "High CPU usage: {}%", cpu_usage);
        }
        
        Timer::after_secs(10).await;
    }
}

fn get_free_memory() -> u32 {
    // 模拟内存查询
    128
}

fn get_cpu_usage() -> u32 {
    // 模拟 CPU 使用率
    45
}

// ============================================================================
// 示例 9: 条件日志（只在特定条件下记录）
// ============================================================================

#[task]
async fn conditional_logging_task() {
    let mut event_counter = 0;
    
    loop {
        event_counter += 1;
        
        // 只记录每 100 次事件
        if event_counter % 100 == 0 {
            log_info!("Events", "Processed {} events", event_counter);
        }
        
        // 只在调试模式下记录详细信息
        #[cfg(debug_assertions)]
        if event_counter % 10 == 0 {
            log_debug!("Events", "Debug: event #{}", event_counter);
        }
        
        Timer::after_millis(10).await;
    }
}

// ============================================================================
// 示例 10: 格式化技巧
// ============================================================================

#[task]
async fn formatting_examples() {
    // 基本格式化
    let value = 42;
    log_info!("Format", "Integer: {}", value);
    
    // 十六进制
    let addr = 0x1000_0000u32;
    log_info!("Format", "Address: 0x{:08X}", addr);
    
    // 二进制
    let flags = 0b1010_1100u8;
    log_info!("Format", "Flags: 0b{:08b}", flags);
    
    // 浮点数
    let voltage = 3.3f32;
    log_info!("Format", "Voltage: {:.2}V", voltage);
    
    // 调试格式
    let status = Some("OK");
    log_debug!("Format", "Status: {:?}", status);
    
    // 多个参数
    log_info!(
        "Format", 
        "System: temp={:.1}°C, voltage={:.2}V, current={:.3}A",
        25.5, 3.3, 0.125
    );
}

