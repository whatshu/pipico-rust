# Windows 调试指南

## ❌ 错误："代码 10 - 该设备无法启动"

### 问题分析

根据你的硬件原理图：

1. **UART0 (GPIO0/GPIO1)** → 用于日志输出
   - 需要外部 USB-UART 转换器（如 CH340、FT232、CP2102）
   - 这个才会在 Windows 中显示为 COM 端口

2. **RP2040 USB (USB_DP/USB_DM)** → 用于 USB 设备功能
   - CDC-ACM 虚拟串口
   - HID 键盘和鼠标
   - 需要正确枚举才能工作

## 🔍 步骤 1：检查设备管理器

### 打开设备管理器
```
Win+X → 设备管理器
```

### 查找以下设备：

#### 情况 A：找到"其他设备"或"未知设备"
```
其他设备
  └─ 未知设备 (带黄色感叹号)
```
**原因**：USB 设备枚举失败或驱动缺失

#### 情况 B：找到"端口 (COM 和 LPT)"
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8) (带黄色感叹号)
```
**原因**：驱动程序问题

#### 情况 C：什么都没有
**原因**：
- 设备未连接
- USB 线缆问题
- 固件未正确运行

## 🔧 解决方案

### 方案 1：检查硬件连接（最重要！）

根据你的原理图，你需要：

#### 选项 A：通过 UART0 查看日志（推荐用于调试）

**硬件需求**：
- USB-UART 转换器（CH340、CP2102、FT232等）

**连接方式**：
```
USB-UART 转换器     RP2040
    TX     ────────→  GPIO1 (RX)
    RX     ←────────  GPIO0 (TX)
    GND    ────────  GND
```

**驱动安装**：
- CH340: [下载驱动](http://www.wch.cn/downloads/CH341SER_ZIP.html)
- CP2102: [下载驱动](https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers)
- FT232: [下载驱动](https://ftdichip.com/drivers/vcp-drivers/)

**测试**：
1. 连接 USB-UART 转换器
2. 查看设备管理器，应该出现新的 COM 端口（如 COM3）
3. 使用串口工具连接（波特率 115200）
4. 应该能看到日志输出

#### 选项 B：通过 RP2040 USB 查看（需要 USB 正确工作）

这个才会显示为 COM8，但目前有问题。

### 方案 2：检查固件是否运行

使用 probe-rs 查看 defmt 日志：

```bash
# 在 PowerShell 或 CMD 中
probe-rs run --chip RP2040 target\thumbv6m-none-eabi\release\rp1-embassy
```

如果能看到 defmt 日志输出，说明：
- ✅ 固件正在运行
- ❌ USB 初始化有问题

### 方案 3：简化 USB 配置（暂时禁用 HID）

如果 USB CDC-ACM 无法工作，尝试简化配置：

**编辑 `src/main.rs`**：

找到这段代码：
```rust
// 创建 USB HID (键盘和鼠标)
static HID_HANDLER: StaticCell<usb::hid::HidRequestHandler> = StaticCell::new();
static KEYBOARD_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();
static MOUSE_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();

let hid_handler = HID_HANDLER.init(usb::hid::HidRequestHandler {});
let keyboard_state = KEYBOARD_STATE.init(embassy_usb::class::hid::State::new());
let mouse_state = MOUSE_STATE.init(embassy_usb::class::hid::State::new());

let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);
let mouse = usb::hid::create_mouse_hid(&mut builder, mouse_state);
```

暂时注释掉 HID 部分：
```rust
// 暂时注释掉 HID
/*
static HID_HANDLER: StaticCell<usb::hid::HidRequestHandler> = StaticCell::new();
static KEYBOARD_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();
static MOUSE_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();

let hid_handler = HID_HANDLER.init(usb::hid::HidRequestHandler {});
let keyboard_state = KEYBOARD_STATE.init(embassy_usb::class::hid::State::new());
let mouse_state = MOUSE_STATE.init(embassy_usb::class::hid::State::new());

let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);
let mouse = usb::hid::create_mouse_hid(&mut builder, mouse_state);
*/
```

同时修改运行部分：
```rust
// 之前
embassy_futures::join::join4(usb_fut, cdc_fut, keyboard_fut, mouse_fut).await;

// 改为
embassy_futures::join::join(usb_fut, cdc_fut).await;
// let keyboard_fut = async {}; // 占位
// let mouse_fut = async {}; // 占位
// embassy_futures::join::join4(usb_fut, cdc_fut, keyboard_fut, mouse_fut).await;
```

重新编译和烧录。

### 方案 4：检查 USB 供电

**可能问题**：
- USB 端口供电不足
- USB Hub 问题

**解决**：
1. 直接连接到电脑的 USB 端口（不通过 HUB）
2. 尝试不同的 USB 端口
3. 尝试 USB 3.0 端口（通常供电更稳定）

### 方案 5：安装 Windows USB 驱动

有时 Windows 需要手动安装 CDC-ACM 驱动。

#### 自动方法（推荐）：
使用 Zadig 工具：
1. 下载 [Zadig](https://zadig.akeo.ie/)
2. 运行 Zadig
3. Options → List All Devices
4. 找到你的设备（RP1 Embassy 或 VID:2E8A）
5. 选择驱动：USB Serial (CDC)
6. 点击 "Install Driver" 或 "Replace Driver"

#### 手动方法：
1. 右键点击设备管理器中的问题设备
2. 更新驱动程序 → 浏览计算机以查找驱动程序
3. 从列表中选择 → 端口 (COM 和 LPT)
4. 选择 "USB Serial Device" 或 "USB CDC ACM"

## 🧪 测试步骤

### 测试 1：使用外部 USB-UART（推荐）

这是最可靠的方法查看日志：

1. **硬件连接**：
   ```
   USB-UART → RP2040
   TX → GPIO1
   RX → GPIO0
   GND → GND
   ```

2. **连接串口**：
   - 打开设备管理器，找到 USB-UART 的 COM 口（如 COM3）
   - 使用 PuTTY/TeraTerm 连接
   - 设置：115200, 8, N, 1

3. **查看输出**：
   应该能看到详细的日志输出。

### 测试 2：使用 probe-rs

如果有调试器（如 Raspberry Pi Debug Probe）：

```bash
# 烧录并查看 defmt 日志
probe-rs run --chip RP2040 target\thumbv6m-none-eabi\release\rp1-embassy
```

## 📊 预期结果对比

### 正常工作时的设备管理器

```
端口 (COM 和 LPT)
  ├─ USB Serial Device (COM3)  ← 外部 USB-UART 转换器
  └─ USB Serial Device (COM8)  ← RP2040 CDC-ACM (可能需要驱动)

人体学输入设备
  ├─ HID-compliant keyboard
  └─ HID-compliant mouse

通用串行总线控制器
  └─ USB Composite Device
```

### 当前问题状态

```
其他设备
  └─ 未知设备 (黄色感叹号)
     └─ 错误代码 10
```

## 🎯 推荐的调试流程

### 第 1 步：使用外部 USB-UART（强烈推荐）

**为什么**：
- 可以看到固件是否运行
- 可以看到 USB 初始化日志
- 独立于 RP2040 的 USB 功能

**如何做**：
1. 购买/使用 USB-UART 转换器（淘宝 ￥5-15）
2. 连接到 GPIO0/GPIO1
3. 连接串口查看日志

### 第 2 步：检查日志输出

从日志中查找：
```
[      50ms] [USB ] [INFO ] USB composite device task started
[      55ms] [USB ] [INFO ] USB driver created
[      60ms] [USB ] [INFO ] USB config created (VID:0x2E8A PID:0x000A)
```

如果看到这些日志，说明固件正在运行，问题在 USB 枚举。

### 第 3 步：根据日志调整

- 如果日志停在某个步骤，说明该步骤有问题
- 如果完全没有日志，说明固件未运行或 UART 连接问题

## 💡 常见问题

### Q: 为什么需要外部 USB-UART？

A: 因为：
1. RP2040 的 UART0 (GPIO0/GPIO1) 只是普通的串口信号
2. 电脑无法直接识别 UART 信号
3. 需要 USB-UART 转换器将其转换为 USB 设备

### Q: RP2040 的 USB 在哪里？

A: RP2040 的 USB (USB_DP/USB_DM) 是独立的功能：
- 用于 CDC-ACM 虚拟串口
- 用于 HID 设备
- 需要软件正确配置

### Q: 代码 10 一定是驱动问题吗？

A: 不一定，也可能是：
- USB 描述符错误
- 设备枚举失败
- 供电问题
- 硬件问题

## 📞 进一步帮助

如果以上方法都不行，请提供：

1. **设备管理器截图**：
   - 展开所有相关设备
   - 显示错误信息

2. **probe-rs 输出**：
   ```bash
   probe-rs run --chip RP2040 target\thumbv6m-none-eabi\release\rp1-embassy > log.txt 2>&1
   ```

3. **硬件信息**：
   - 开发板型号
   - USB 连接方式
   - 是否有外部 USB-UART

4. **USB 设备日志**：
   在设备管理器中右键点击设备 → 属性 → 事件 → 查看详细信息

## 🔗 有用的工具

- **Zadig**: https://zadig.akeo.ie/ (USB 驱动安装)
- **USBTreeView**: https://www.uwe-sieber.de/usbtreeview_e.html (查看 USB 设备详情)
- **PuTTY**: https://www.putty.org/ (串口终端)
- **probe-rs**: https://probe.rs/ (固件烧录和调试)

