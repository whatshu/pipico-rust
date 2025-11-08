# 异步日志系统说明

本项目使用完全异步的日志系统，基于 Embassy 框架的 Channel 实现。

## 特性

### ✅ 完全异步
- 日志通过异步 Channel 发送，不会阻塞主任务
- UART 输出任务独立运行，异步处理所有日志消息
- 支持双核（Core 0 和 Core 1）同时写入日志

### ✅ 两种日志宏

#### 1. 异步日志宏（推荐）
适用于 async 函数中：
- `log_info!("CoreName", "message")`
- `log_debug!("CoreName", "message")`
- `log_warn!("CoreName", "message")`
- `log_error!("CoreName", "message")`

**特点**：
- 会等待直到消息成功放入队列
- 保证消息不丢失
- 只能在 async 上下文中使用

#### 2. 同步日志宏（非阻塞）
适用于非 async 上下文：
- `log_info_sync!("CoreName", "message")`
- `log_debug_sync!("CoreName", "message")`
- `log_warn_sync!("CoreName", "message")`
- `log_error_sync!("CoreName", "message")`

**特点**：
- 立即返回，不会等待
- 如果队列满了会丢弃消息
- 可以在任何上下文中使用

## 使用示例

### 在异步任务中使用

```rust
#[embassy_executor::task]
async fn my_task() {
    // 异步日志，会等待发送成功
    log_info!("Task", "Task started");
    
    let counter = 42;
    log_debug!("Task", "Counter value: {}", counter);
    
    // 错误日志
    if let Err(e) = do_something().await {
        log_error!("Task", "Operation failed: {:?}", e);
    }
}
```

### 在中断或同步上下文中使用

```rust
fn some_sync_function() {
    // 同步日志，非阻塞
    log_info_sync!("Sync", "Function called");
    
    // 这不会阻塞，即使队列满了
    for i in 0..100 {
        log_debug_sync!("Sync", "Loop iteration: {}", i);
    }
}
```

## 日志格式

所有日志都包含以下信息：
```
[  123456ms] [CoreName] [LEVEL] Message content
```

- **时间戳**：从系统启动开始的毫秒数
- **Core名称**：自定义的标识符（如 "Core0", "USB", "Task" 等）
- **日志级别**：INFO, DEBUG, WARN, ERROR
- **消息内容**：格式化的日志消息

### 示例输出
```
[      100ms] [Core0] [INFO ] System initialization complete
[      150ms] [USB  ] [INFO ] USB composite device initialized (Serial + HID)
[     1234ms] [Core1] [DEBUG] Checkpoint: 10
[     5678ms] [Task ] [ERROR] Connection failed: timeout
```

## 配置

### 队列大小
在 `src/logger.rs` 中可以调整队列大小：

```rust
pub static CHANNEL: Channel<CriticalSectionRawMutex, String<256>, 16> = Channel::new();
                                                                    ^^
                                                        队列深度（消息数量）
```

- **默认值**：16 条消息
- **建议值**：根据日志频率调整
  - 高频日志：32 或更多
  - 低频日志：8-16 即可

### 消息长度
当前每条消息最大 256 字节：

```rust
String<256>
       ^^^
  最大字节数
```

如果需要更长的消息，可以增加这个值，但会占用更多内存。

## 架构设计

```
┌─────────────┐         ┌──────────────────┐         ┌─────────────┐
│  Core 0/1   │         │   Async Channel  │         │  UART Task  │
│   Tasks     │ ──────> │   (16 entries)   │ ──────> │             │
│             │ async   │                  │  async  │  Serial Out │
└─────────────┘ send    └──────────────────┘ receive └─────────────┘
                                                             │
                                                             v
                                                       USB Serial / 
                                                          UART0
```

### 工作流程

1. **任务记录日志**：调用日志宏
2. **格式化消息**：生成带时间戳的日志字符串
3. **发送到 Channel**：异步或同步方式
4. **UART 任务接收**：从 Channel 中取出消息
5. **串口输出**：通过 UART 发送到终端

## 性能考虑

### 内存使用
- 每条日志：256 字节
- 队列深度：16 条
- **总内存**：256 × 16 = 4KB（用于日志缓冲）

### 延迟
- **异步发送**：如果队列未满，几乎无延迟；队列满时会等待
- **同步发送**：立即返回（<1µs）
- **UART 输出**：取决于波特率（115200 bps 约 1ms/128字节）

### 吞吐量
在 115200 波特率下：
- 理论速度：~11.5 KB/s
- 实际速度：~10 KB/s（考虑开销）
- 约每秒 40 条日志消息（256字节/条）

## 最佳实践

### 1. 选择合适的日志级别
```rust
// INFO - 重要的状态变化
log_info!("System", "Device initialized");

// DEBUG - 调试信息（可在发布版本中禁用）
log_debug!("Module", "Internal state: {}", state);

// WARN - 警告（可恢复的错误）
log_warn!("Network", "Connection unstable, retrying...");

// ERROR - 错误（需要注意的问题）
log_error!("Driver", "Hardware fault detected");
```

### 2. 异步 vs 同步
```rust
// 在 async 函数中 - 使用异步版本
async fn async_task() {
    log_info!("Task", "Starting");  // ✅ 推荐
}

// 在中断或非 async 上下文 - 使用同步版本
fn interrupt_handler() {
    log_info_sync!("ISR", "Interrupt occurred");  // ✅ 推荐
}
```

### 3. 避免过度日志
```rust
// ❌ 不好 - 高频循环中记录日志
loop {
    log_debug!("Loop", "Iteration");  // 会快速填满队列
    Timer::after_millis(1).await;
}

// ✅ 好 - 选择性记录
let mut counter = 0;
loop {
    counter += 1;
    if counter % 1000 == 0 {
        log_debug!("Loop", "Processed {} iterations", counter);
    }
    Timer::after_millis(1).await;
}
```

### 4. 格式化技巧
```rust
// 使用 Rust 格式化语法
log_info!("Module", "Value: {}", value);
log_info!("Module", "Hex: 0x{:08X}", addr);
log_info!("Module", "Debug: {:?}", structure);

// 避免过长的消息（超过 256 字节会被截断）
log_info!("Module", "Very long message...");  // 确保 < 256 字节
```

## 故障排除

### 日志丢失
**原因**：队列已满（使用同步宏时）
**解决**：
1. 增加队列大小
2. 降低日志频率
3. 使用异步宏（会等待）

### 日志延迟
**原因**：UART 输出速度慢
**解决**：
1. 提高波特率（在 `config.rs` 中设置）
2. 减少日志输出量
3. 缩短日志消息

### 编译错误：cannot find macro
**原因**：宏未正确导入
**解决**：
```rust
// 在文件顶部添加
use crate::{log_info, log_debug, log_warn, log_error};
// 或同步版本
use crate::{log_info_sync, log_debug_sync};
```

## 与 defmt 的对比

本项目同时使用：
- **defmt**：用于调试输出（probe-rs）
- **异步日志**：用于串口输出

```rust
// defmt - 只在调试器中可见
info!("This goes to the debugger");

// 异步日志 - 输出到串口
log_info!("Core0", "This goes to UART/USB Serial");
```

## 未来改进

可能的增强功能：
- [ ] 日志级别过滤（编译时或运行时）
- [ ] 彩色日志输出（ANSI 转义码）
- [ ] 日志统计（丢弃计数）
- [ ] 多个输出目标（USB + UART）
- [ ] 日志压缩

## 许可证

遵循项目主许可证。

