# 项目架构文档

## 模块结构

```
src/
├── main.rs          # 主程序入口 (71 行)
├── logger.rs        # 日志系统 (91 行)
├── config.rs        # 配置常量 (32 行)
├── banner.rs        # 启动横幅 (27 行)
└── tasks/           # 任务模块
    ├── mod.rs       # 模块导出 (10 行)
    ├── core0.rs     # Core 0 任务 (29 行)
    └── core1.rs     # Core 1 任务 (29 行)
```

**总计**: 289 行代码

## 模块详解

### 1. main.rs - 主程序入口

**职责**:
- 系统初始化
- UART 配置和初始化
- 启动各个异步任务
- 双核协调

**关键代码**:
```rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 初始化外设
    let p = embassy_rp::init(Default::default());
    
    // 配置 UART
    let uart = Uart::new(...);
    
    // 启动任务
    spawner.spawn(uart_task(uart)).unwrap();
    spawner.spawn(core0_task()).unwrap();
    spawn_core1(...);
}
```

### 2. logger.rs - 日志系统

**职责**:
- 提供统一的日志接口
- 管理日志消息通道
- 实现串口输出任务

**导出的 API**:
```rust
// Channel 用于跨核通信
pub static CHANNEL: Channel<...> = ...;

// 日志宏
log_info!("Core0", "message");
log_debug!("Core0", "value={}", x);
log_warn!("Core0", "warning");
log_error!("Core0", "error");

// UART 任务
pub async fn uart_task(uart: Uart<...>) { ... }
```

**日志格式**:
```
[{uptime_ms}] [{core}] [{level}] {message}
```

### 3. config.rs - 配置管理

**职责**:
- 集中管理所有配置常量
- 便于调整系统参数

**配置项**:
```rust
pub mod uart {
    pub const BAUD_RATE: u32 = 115200;
    pub const TX_PIN: u8 = 0;
    pub const RX_PIN: u8 = 1;
}

pub mod task {
    pub const CORE0_INTERVAL_MS: u64 = 1000;
    pub const CORE1_INTERVAL_MS: u64 = 1500;
    pub const CORE0_MILESTONE: u32 = 10;
    pub const CORE1_CHECKPOINT: u32 = 5;
}

pub const CORE1_STACK_SIZE: usize = 4096;
```

### 4. banner.rs - 启动横幅

**职责**:
- 生成系统启动信息
- 显示配置参数

**输出示例**:
```
=====================================
  RP2040 Dual Core UART Demo
  Embassy Async Framework
=====================================
UART0 Config:
  - Baud Rate: 115200
  - TX: GPIO0, RX: GPIO1
  - Data: 8N1
=====================================
```

### 5. tasks/ - 任务模块

#### tasks/mod.rs
- 导出所有任务函数
- 作为任务模块的统一入口

#### tasks/core0.rs
**Core 0 任务**:
- 每 1 秒输出心跳
- 每 10 个计数输出里程碑 DEBUG 日志

```rust
#[task]
pub async fn core0_task() {
    loop {
        log_info!("Core0", "Heartbeat, count={}", counter);
        if counter % 10 == 0 {
            log_debug!("Core0", "Milestone: {}", counter);
        }
        Timer::after(Duration::from_millis(1000)).await;
    }
}
```

#### tasks/core1.rs
**Core 1 任务**:
- 每 1.5 秒输出心跳
- 每 5 个计数输出检查点 DEBUG 日志

```rust
#[task]
pub async fn core1_task() {
    loop {
        log_info!("Core1", "Heartbeat, count={}", counter);
        if counter % 5 == 0 {
            log_debug!("Core1", "Checkpoint: {}", counter);
        }
        Timer::after(Duration::from_millis(1500)).await;
    }
}
```

## 数据流

```
┌─────────────────────────────────────────────────┐
│                   Core 0                        │
│  ┌──────────┐         ┌──────────────┐        │
│  │ main()   │────────▶│ core0_task() │        │
│  └──────────┘         └──────┬───────┘        │
│                              │                  │
│                        log_info!()              │
│                              │                  │
│  ┌──────────────┐           ▼                  │
│  │ uart_task()  │◀──── CHANNEL ◀───┐          │
│  └──────┬───────┘                    │          │
│         │                             │          │
│         ▼                             │          │
│    UART TX (GPIO0)                   │          │
└─────────────────────────────────────┼──────────┘
                                        │
┌─────────────────────────────────────┼──────────┐
│                   Core 1             │          │
│  ┌──────────────┐                    │          │
│  │ core1_task() │──────────────────┘          │
│  └──────────────┘   log_info!()                │
│                                                  │
└─────────────────────────────────────────────────┘
```

## 扩展指南

### 添加新任务

1. 在 `tasks/` 下创建新文件 `new_task.rs`
2. 实现任务函数:
```rust
use embassy_executor::task;

#[task]
pub async fn new_task() {
    loop {
        log_info!("NewTask", "Running");
        Timer::after(Duration::from_secs(1)).await;
    }
}
```

3. 在 `tasks/mod.rs` 中导出:
```rust
pub mod new_task;
pub use new_task::new_task;
```

4. 在 `main.rs` 中启动:
```rust
spawner.spawn(new_task()).unwrap();
```

### 添加新配置

在 `config.rs` 中添加:
```rust
pub mod new_feature {
    pub const PARAM1: u32 = 100;
    pub const PARAM2: bool = true;
}
```

### 修改日志格式

编辑 `logger.rs` 中的宏定义，修改格式字符串。

## 依赖关系

```
main.rs
  ├── logger (CHANNEL, uart_task)
  ├── config (uart::*, task::*, CORE1_STACK_SIZE)
  ├── banner (send_banner)
  └── tasks (core0_task, core1_task)

tasks/core0.rs
  ├── logger (log_info!, log_debug!)
  └── config (task::*)

tasks/core1.rs
  ├── logger (log_info!, log_debug!)
  └── config (task::*)

banner.rs
  ├── logger (CHANNEL)
  └── config (uart::BAUD_RATE)
```

## 编译结果

- **二进制大小**: 569K (release, opt-level="z")
- **Flash 占用**: ~37KB UF2
- **代码行数**: 289 行
- **模块数**: 7 个文件

## 最佳实践

1. **单一职责**: 每个模块只负责一个功能
2. **配置集中**: 所有魔法数字放在 config.rs
3. **文档注释**: 为公共 API 添加 `///` 注释
4. **错误处理**: 使用 `unwrap()` 明确标记不应失败的操作
5. **异步优先**: 所有耗时操作使用 async/await

