# 故障排除指南

## 问题：没有日志输出，USB 设备未被检测

### 已修复的问题

1. **日志初始化顺序错误** ✅ 已修复
   - 问题：`send_banner()` 在 UART 任务启动前调用
   - 修复：先启动 UART 任务，等待 10ms 后再发送 banner

2. **添加了详细的调试日志** ✅ 已完成
   - 每个初始化步骤都有日志输出
   - USB 枚举过程有详细日志
   - 各个 USB 类的运行状态有日志

### 检查步骤

#### 1. 硬件连接检查

根据原理图，确认以下连接：

**UART0 连接（用于日志输出）**：
- GPIO0 (PIN_0) → TX
- GPIO1 (PIN_1) → RX
- 波特率：115200
- 数据格式：8N1

**USB 连接**：
- USB_DP 和 USB_DM 应正确连接到 RP2040
- USB 电源应正常供电
- 确认 USB 数据线不是只有充电功能的线（必须是数据线）

**调试接口（SWD）**：
- SWCLK 和 SWDIO 正确连接到调试器

#### 2. 烧录固件

```bash
# 构建发布版本
cargo build --release

# 烧录到设备（使用 probe-rs）
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy

# 或者使用 Makefile
make flash
```

#### 3. 查看日志输出

**方法 1：使用串口终端（推荐查看详细日志）**
```bash
# Linux/Mac
screen /dev/ttyUSB0 115200
# 或
minicom -D /dev/ttyUSB0 -b 115200

# Windows
# 使用 PuTTY 或 TeraTerm
# COM端口：检查设备管理器
# 波特率：115200
```

**方法 2：使用 probe-rs（查看 defmt 日志）**
```bash
# 运行并查看 defmt 日志
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy
```

#### 4. 预期的日志输出

正常启动应该看到类似以下的日志：

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

[      10ms] [Main] [INFO ] System initialization starting...
[      15ms] [Main] [INFO ] Initializing USB composite device...
[      20ms] [USB ] [INFO ] USB composite device task started
[      25ms] [USB ] [INFO ] USB driver created
[      30ms] [USB ] [INFO ] USB config created (VID:0x2E8A PID:0x000A)
[      35ms] [USB ] [INFO ] USB builder created
[      40ms] [USB ] [INFO ] CDC-ACM serial port created
[      45ms] [USB ] [INFO ] HID keyboard and mouse created
[      50ms] [USB ] [INFO ] USB composite device built successfully
[      55ms] [USB ] [INFO ] Waiting for USB enumeration...
[      60ms] [USB-Serial] [INFO ] CDC-ACM task running
[      65ms] [USB-Serial] [INFO ] Waiting for connection...
[      70ms] [USB-Keyboard] [INFO ] HID Keyboard task running
[      75ms] [USB-Keyboard] [INFO ] Will send 'H' key every 5 seconds
[      80ms] [USB-Mouse] [INFO ] HID Mouse task running
[      85ms] [USB-Mouse] [INFO ] Will move mouse every 3 seconds
[      90ms] [Main] [INFO ] Spawning Core 0 task
[      95ms] [Main] [INFO ] Spawning Core 1
[     100ms] [Core0] [INFO ] Task started
[     105ms] [Core1] [INFO ] Task started
[     150ms] [Main] [INFO ] ====================================
[     155ms] [Main] [INFO ] System startup complete!
[     160ms] [Main] [INFO ] - UART0: GPIO0(TX) / GPIO1(RX)
[     165ms] [Main] [INFO ] - USB: CDC-ACM + HID Keyboard + HID Mouse
[     170ms] [Main] [INFO ] - Dual Core: Core0 + Core1 running
[     175ms] [Main] [INFO ] ====================================
[    1000ms] [Core0] [INFO ] Heartbeat, count=0
[    1500ms] [Core1] [INFO ] Heartbeat, count=0
[    2000ms] [Core0] [INFO ] Heartbeat, count=1
...
```

### 常见问题

#### 问题 1: 完全没有任何输出

**可能原因**：
1. UART 连接错误
2. 波特率设置不正确
3. 固件未成功烧录
4. 硬件未正常启动

**解决方法**：
1. **检查 UART 连接**：
   - 确认 TX/RX 连接正确（注意交叉连接）
   - 确认 GND 已连接
   
2. **检查串口设置**：
   - 波特率：115200
   - 数据位：8
   - 停止位：1
   - 校验：无

3. **使用 probe-rs 查看 defmt 日志**：
   ```bash
   probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/rp1-embassy
   ```
   如果能看到 defmt 日志但看不到 UART 日志，说明 UART 连接有问题。

4. **检查固件是否运行**：
   - 板上LED是否闪烁（如果有）
   - 使用 `probe-rs info` 检查芯片状态

#### 问题 2: USB 设备未被检测

**可能原因**：
1. USB 数据线只支持充电（没有数据线）
2. USB 端口供电不足
3. 硬件 USB 电路问题
4. 驱动程序未安装（Windows）

**解决方法**：

1. **检查 USB 数据线**：
   - 确保使用支持数据传输的 USB 线
   - 尝试更换另一根 USB 线

2. **检查 USB 端口**：
   - 尝试不同的 USB 端口
   - 直接连接到主机（不要通过 HUB）
   - 确保 USB 端口供电充足

3. **查看系统日志**：

   **Linux**:
   ```bash
   # 查看 USB 枚举日志
   dmesg | tail -50
   
   # 应该看到类似：
   # usb 1-1: new full-speed USB device number X using xhci_hcd
   # usb 1-1: New USB device found, idVendor=2e8a, idProduct=000a
   # cdc_acm 1-1:1.0: ttyACM0: USB ACM device
   ```

   **Windows**:
   - 打开设备管理器
   - 查看"端口(COM和LPT)"部分
   - 应该看到新的 COM 端口

   **macOS**:
   ```bash
   # 查看 USB 设备
   system_profiler SPUSBDataType
   
   # 查看串口设备
   ls /dev/tty.usb*
   ```

4. **检查 VID/PID**：
   当前配置：
   - VID: 0x2E8A (Raspberry Pi)
   - PID: 0x000A (自定义)

   如果需要修改，编辑 `src/usb/mod.rs`：
   ```rust
   pub const USB_VID: u16 = 0x2e8a;
   pub const USB_PID: u16 = 0x000a;
   ```

#### 问题 3: USB 串口连接后立即断开

**可能原因**：
1. USB 枚举失败
2. 描述符错误
3. 电源问题

**解决方法**：

1. **查看详细日志**：
   - 检查 UART 输出的 USB 相关日志
   - 查看是否有错误消息

2. **降低 USB 功耗**：
   编辑 `src/usb/mod.rs`：
   ```rust
   config.max_power = 100; // 改为 200mA (100 * 2mA)
   ```

3. **简化 USB 配置**：
   暂时只启用 CDC-ACM，禁用 HID：
   - 注释掉 HID 相关代码
   - 只保留 CDC-ACM

#### 问题 4: 部分功能正常，部分不正常

**USB 串口正常但 HID 不工作**：
- 检查 HID 驱动是否正确加载
- 查看 USB 日志中的错误信息
- 尝试重新插拔 USB

**HID 正常但串口不工作**：
- 检查串口终端配置
- 确认 COM 端口号正确
- 尝试发送数据测试回显

### 调试技巧

#### 1. 使用 defmt 日志

在代码中添加更多 `info!()`, `debug!()`, `warn!()`, `error!()` 调用：

```rust
info!("Checkpoint A");
debug!("Variable value: {}", value);
warn!("Something unusual");
error!("Critical error!");
```

#### 2. 使用 LED 指示

如果板上有 LED，可以添加 LED 闪烁代码来指示状态：

```rust
// 在 main.rs 中
let mut led = Output::new(p.PIN_25, Level::Low);
loop {
    led.set_high();
    Timer::after_millis(100).await;
    led.set_low();
    Timer::after_millis(100).await;
}
```

#### 3. 分步调试

逐步启用功能：
1. 先只启用 UART 日志
2. 然后添加 Core0/Core1 任务
3. 最后添加 USB 功能

#### 4. 使用逻辑分析仪

如果有逻辑分析仪：
- 监控 USB_DP/USB_DM 信号
- 监控 UART TX/RX 信号
- 查看波形是否正常

### 获取帮助

如果以上方法都无法解决问题，请提供：

1. **硬件信息**：
   - 板子型号
   - RP2040 版本
   - USB 连接方式

2. **日志输出**：
   - 完整的 UART 日志
   - defmt 日志（如果有）
   - 系统 USB 枚举日志（dmesg 或设备管理器）

3. **操作步骤**：
   - 详细的操作步骤
   - 出现问题的具体时刻
   - 任何错误消息

4. **编译信息**：
   ```bash
   cargo --version
   rustc --version
   probe-rs --version
   ```

### 参考资料

- [Embassy 官方文档](https://embassy.dev/)
- [RP2040 数据手册](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
- [USB CDC-ACM 规范](https://www.usb.org/document-library/class-definitions-communication-devices-12)
- [USB HID 规范](https://www.usb.org/hid)

