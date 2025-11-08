# 快速入门指南

## 🎯 项目概述

这是一个基于 Embassy 异步框架的 RP2040 双核项目，包含：

- ✅ **异步日志系统**：通过 UART0 输出详细日志
- ✅ **USB 复合设备**：CDC-ACM 串口 + HID 键盘 + HID 鼠标
- ✅ **双核支持**：Core 0 和 Core 1 独立运行任务
- ✅ **完整的调试支持**：defmt + UART 日志

## 🔧 硬件要求

### 必需连接

1. **UART0（用于日志输出）**
   - GPIO0 (PIN 1) → UART TX
   - GPIO1 (PIN 2) → UART RX
   - GND → GND
   - 波特率：115200 8N1

2. **USB（用于 USB 功能）**
   - USB_DP 和 USB_DM 连接正常
   - USB 数据线（非仅充电线）

3. **调试接口（可选，用于 defmt 日志）**
   - SWCLK → 调试器
   - SWDIO → 调试器
   - GND → GND

## 🚀 快速开始

### 步骤 1：构建项目

```bash
# 开发版本（带调试信息）
cargo build

# 发布版本（优化大小）
cargo build --release
```

### 步骤 2：烧录固件

#### 方法 A：使用 probe-rs（推荐）

```bash
# 烧录并查看 defmt 日志
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy
```

#### 方法 B：使用 Makefile

```bash
# 烧录发布版本
make flash

# 烧录调试版本
make flash-debug
```

#### 方法 C：手动烧录

```bash
# 1. 按住 BOOTSEL 按钮
# 2. 连接 USB
# 3. 释放 BOOTSEL（设备应显示为 USB 存储设备）
# 4. 复制 UF2 文件到设备
```

### 步骤 3：查看日志输出

#### 方法 A：UART 日志（推荐用于查看详细日志）

**Linux/Mac:**
```bash
# 使用 screen
screen /dev/ttyUSB0 115200

# 或使用测试脚本
./scripts/test_uart.sh

# 或使用 minicom
minicom -D /dev/ttyUSB0 -b 115200
```

**Windows:**
- 使用 PuTTY、TeraTerm 或 Arduino Serial Monitor
- 串口设置：115200, 8, N, 1

#### 方法 B：defmt 日志（用于开发调试）

```bash
# 在 probe-rs run 时自动显示
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/rp1-embassy
```

## 📊 预期输出

### 成功启动的日志示例

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

[      10ms] [Main] [INFO ] System initialization starting...
[      15ms] [Main] [INFO ] Initializing USB composite device...
[      50ms] [USB ] [INFO ] USB composite device task started
[      55ms] [USB ] [INFO ] USB driver created
[      60ms] [USB ] [INFO ] USB config created (VID:0x2E8A PID:0x000A)
[      65ms] [USB ] [INFO ] USB builder created
[      70ms] [USB ] [INFO ] CDC-ACM serial port created
[      75ms] [USB ] [INFO ] HID keyboard and mouse created
[      80ms] [USB ] [INFO ] USB composite device built successfully
[      85ms] [USB-Serial] [INFO ] CDC-ACM task running
[      90ms] [USB-Serial] [INFO ] Waiting for connection...
[      95ms] [USB-Keyboard] [INFO ] HID Keyboard task running
[     100ms] [USB-Mouse] [INFO ] HID Mouse task running
[     150ms] [Main] [INFO ] ====================================
[     155ms] [Main] [INFO ] System startup complete!
[     160ms] [Main] [INFO ] - UART0: GPIO0(TX) / GPIO1(RX)
[     165ms] [Main] [INFO ] - USB: CDC-ACM + HID Keyboard + HID Mouse
[     170ms] [Main] [INFO ] - Dual Core: Core0 + Core1 running
[     175ms] [Main] [INFO ] ====================================
[    1000ms] [Core0] [INFO ] Heartbeat, count=0
[    1500ms] [Core1] [INFO ] Heartbeat, count=1
...
```

### USB 设备检测

**Linux:**
```bash
# 查看 USB 设备
lsusb | grep 2e8a

# 应该显示：
# Bus 001 Device 010: ID 2e8a:000a RP1 Embassy Composite Device

# 查看串口设备
ls /dev/ttyACM*
# 应该有新的 /dev/ttyACM0
```

**Windows:**
- 设备管理器中应显示：
  - "端口 (COM 和 LPT)" 下有新的 COM 端口
  - "人体学输入设备" 下有键盘和鼠标

**macOS:**
```bash
# 查看串口设备
ls /dev/tty.usb*
```

## 🧪 功能测试

### 1. 测试 USB 串口（CDC-ACM）

```bash
# Linux/Mac
echo "Hello RP2040" > /dev/ttyACM0
cat /dev/ttyACM0

# 应该会回显你发送的内容
```

### 2. 测试 HID 键盘

- 每 5 秒会自动发送一个 'H' 键
- 在任何文本编辑器中应该能看到

### 3. 测试 HID 鼠标

- 每 3 秒会自动移动鼠标
- 鼠标指针应该会向右下移动

### 4. 测试双核日志

查看 UART 日志，应该能看到：
- `[Core0]` 的心跳日志（每 1 秒）
- `[Core1]` 的心跳日志（每 1.5 秒）

## ⚙️ 配置

### 修改 UART 配置

编辑 `src/config.rs`:

```rust
pub mod uart {
    pub const BAUD_RATE: u32 = 115200;  // 修改波特率
    pub const TX_PIN: u8 = 0;            // 修改 TX 引脚
    pub const RX_PIN: u8 = 1;            // 修改 RX 引脚
}
```

### 修改 USB 配置

编辑 `src/usb/mod.rs`:

```rust
pub const USB_VID: u16 = 0x2e8a;         // 修改 VID
pub const USB_PID: u16 = 0x000a;         // 修改 PID
pub const USB_MANUFACTURER: &str = "你的名称";
pub const USB_PRODUCT: &str = "你的产品名";
```

### 修改任务间隔

编辑 `src/config.rs`:

```rust
pub mod task {
    pub const CORE0_INTERVAL_MS: u64 = 1000;   // Core 0 间隔
    pub const CORE1_INTERVAL_MS: u64 = 1500;   // Core 1 间隔
}
```

### 修改 HID 行为

编辑 `src/usb/hid.rs`:

```rust
// 修改键盘发送间隔
Timer::after_secs(5).await;  // 改为你想要的间隔

// 修改发送的键码
let report = [0, 0, 0x0B, 0, 0, 0, 0, 0];  // 0x0B = 'H'
// 键码参考：https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf

// 修改鼠标移动
let report = [0, 10, 10, 0];  // buttons, x, y, wheel
```

## 🐛 故障排除

### 问题：没有任何日志输出

1. **检查 UART 连接**
   - TX/RX 引脚是否正确
   - 波特率是否为 115200
   - GND 是否连接

2. **检查串口设置**
   ```bash
   # 查看可用串口
   ls /dev/tty*
   ```

3. **查看详细故障排除**
   - 参考 `TROUBLESHOOTING.md`

### 问题：USB 设备未被检测

1. **检查 USB 数据线**
   - 确保不是只有充电功能的线

2. **查看系统日志**
   ```bash
   # Linux
   dmesg | tail -20
   
   # 应该看到 USB 枚举信息
   ```

3. **查看详细故障排除**
   - 参考 `TROUBLESHOOTING.md`

## 📚 文档

- `LOG_ASYNC_README.md` - 异步日志系统详细说明
- `USB_README.md` - USB 功能详细说明
- `TROUBLESHOOTING.md` - 完整故障排除指南
- `examples_log_usage.rs` - 日志使用示例代码

## 🔗 有用的命令

```bash
# 构建
cargo build --release

# 检查代码（不构建）
cargo check

# 格式化代码
cargo fmt

# 清理构建文件
cargo clean

# 查看二进制大小
cargo size --release

# 烧录（使用 probe-rs）
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy

# 擦除芯片
probe-rs erase --chip RP2040

# 查看芯片信息
probe-rs info
```

## 📝 开发技巧

### 1. 使用异步日志

```rust
// 在 async 函数中
log_info!("MyTask", "Message");

// 在同步/中断上下文中
log_info_sync!("ISR", "Message");
```

### 2. 添加新任务

```rust
#[embassy_executor::task]
async fn my_task() {
    log_info!("MyTask", "Task started");
    
    loop {
        // 你的代码
        Timer::after_secs(1).await;
    }
}

// 在 main 中启动
spawner.spawn(my_task()).unwrap();
```

### 3. 调试技巧

```rust
// 使用 defmt 宏（在 probe-rs 中显示）
info!("Debug info");
debug!("Detailed info");
warn!("Warning");
error!("Error!");

// 使用异步日志（在 UART 中显示）
log_info!("Core", "Message");
```

## 🎓 学习资源

- [Embassy 官方文档](https://embassy.dev/)
- [RP2040 数据手册](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
- [Rust 嵌入式之书](https://docs.rust-embedded.org/book/)

## 📄 许可证

遵循项目主许可证。

