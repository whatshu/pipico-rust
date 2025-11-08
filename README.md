# RP1-Embassy: RP2040 双核串口演示

基于 Embassy 异步框架的 RP2040 双核串口输出演示项目。

## 功能

- 双核异步执行器 - 两个核心都使用 Embassy 异步框架
- UART0 串口输出 (115200 baud)
- Core 0: 每秒输出一次计数器
- Core 1: 每 1.5 秒输出一次计数器
- 使用 Channel 在双核间共享串口输出
- 完整的异步/await 语法支持

## 硬件连接

- **UART0 TX**: GPIO0
- **UART0 RX**: GPIO1
- **波特率**: 115200
- **数据位**: 8
- **停止位**: 1
- **校验**: None

### 串口连接方式

使用 USB-TTL 转换器连接：
- TX (GPIO0) → USB-TTL RX
- RX (GPIO1) → USB-TTL TX (可选，本项目只输出)
- GND → USB-TTL GND

## 硬件要求

- Raspberry Pi Pico 或其他 RP2040 开发板
- USB-TTL 串口转换器

## 快速开始

### 构建项目

```bash
make build
```

这将自动：
1. 编译项目 (release 模式)
2. 生成 `rp1-embassy.uf2` 文件

### 烧录到设备

1. 按住 Pico 的 BOOTSEL 按钮，连接到电脑
2. Pico 会显示为 USB 存储设备
3. 将 `rp1-embassy.uf2` 文件复制到该设备
4. Pico 自动重启并运行程序

### 查看串口输出

使用串口终端工具连接（波特率 115200）：

**Linux/macOS:**
```bash
# 查找串口设备
ls /dev/ttyUSB* /dev/ttyACM*

# 使用 screen
screen /dev/ttyUSB0 115200

# 或使用 minicom
minicom -D /dev/ttyUSB0 -b 115200
```

**Windows:**
```
使用 PuTTY 或 Tera Term
- 端口: COM3 (根据实际情况)
- 波特率: 115200
```

### 预期输出

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
Log Format:
  [uptime_ms] [Core] [LEVEL] message
=====================================

[       0ms] [Core0] [INFO ] Initializing Core 0 executor
[       1ms] [Core0] [INFO ] System startup complete
[       2ms] [Core0] [INFO ] Task started
[       3ms] [Core1] [INFO ] Task started
[       4ms] [Core0] [INFO ] Heartbeat, count=0
[    1004ms] [Core0] [INFO ] Heartbeat, count=1
[    1505ms] [Core1] [INFO ] Heartbeat, count=1
[    2005ms] [Core0] [INFO ] Heartbeat, count=2
[    3005ms] [Core0] [INFO ] Heartbeat, count=3
[    3006ms] [Core1] [INFO ] Heartbeat, count=2
[    4006ms] [Core0] [INFO ] Heartbeat, count=4
...
[   10010ms] [Core0] [INFO ] Heartbeat, count=10
[   10010ms] [Core0] [DEBUG] Milestone reached: 10
...
```

**日志格式说明：**
- `[uptime_ms]`: 系统运行时间（毫秒）
- `[Core]`: Core0 或 Core1
- `[LEVEL]`: INFO, DEBUG, WARN, ERROR
- `message`: 日志消息内容

## 核心依赖

- embassy-executor: 异步执行器
- embassy-rp: RP2040 HAL
- embassy-time: 时间和定时器
- defmt: 高效日志框架

## 项目结构

```
rp1-embassy/
├── src/main.rs           # 主程序
├── Cargo.toml            # 依赖配置
├── .cargo/config.toml    # 构建配置
├── memory.x              # 内存布局
├── build.rs              # 构建脚本
└── Makefile              # 构建工具
```

