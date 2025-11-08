# USB 复合设备功能说明

本项目已成功集成 USB 复合设备支持，包括以下功能：

## 已实现的 USB 功能

### 1. USB 串口 (CDC-ACM)
- **状态**: ✅ 已实现
- **功能**: 模拟一个 USB 串行端口，可用于与主机进行通信
- **特性**: 
  - 自动回显接收到的数据
  - 64字节包大小
  - 自动检测连接/断开

### 2. USB HID - 键盘
- **状态**: ✅ 已实现
- **功能**: 模拟 USB 键盘
- **特性**:
  - 每 5 秒自动发送一个 'H' 键
  - 支持完整的键盘报告格式
  - 8字节 HID 报告

### 3. USB HID - 鼠标
- **状态**: ✅ 已实现
- **功能**: 模拟 USB 鼠标
- **特性**:
  - 每 3 秒自动移动鼠标（向右下移动 10 像素）
  - 4字节 HID 报告
  - 支持按钮、XY移动和滚轮

### 4. USB 大容量存储 (MSC/U盘)
- **状态**: ⚠️ 待实现
- **说明**: embassy-usb 0.3.0 目前不支持 MSC 类
- **占位符**: 已创建占位符代码结构，等待未来版本支持

## 技术细节

### USB 配置
- **VID**: 0x2e8a (Raspberry Pi)
- **PID**: 0x000a (自定义)
- **制造商**: "RP1 Embassy"
- **产品名称**: "Composite Device"
- **最大功率**: 500mA

### 设备类型
- 复合设备配置 (Device Class: 0xEF, SubClass: 0x02, Protocol: 0x01)
- 支持多个 USB 接口同时工作

## 代码结构

```
src/usb/
├── mod.rs       # USB 模块主文件，包含通用配置
├── serial.rs    # USB 串口实现 (CDC-ACM)
├── hid.rs       # USB HID 实现 (键盘和鼠标)
└── storage.rs   # USB 存储设备占位符
```

## 使用方法

### 1. 构建项目
```bash
cargo build --release
```

### 2. 烧录到设备
```bash
make flash
# 或
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy
```

### 3. 连接到主机
设备连接后，主机应该识别出：
- 一个 CDC-ACM 串口设备 (例如 /dev/ttyACM0 或 COM3)
- 一个 HID 键盘设备
- 一个 HID 鼠标设备

### 4. 测试 USB 串口
在 Linux/Mac 上：
```bash
# 打开串口
screen /dev/ttyACM0 115200
# 输入任何字符，应该会回显
```

在 Windows 上可以使用 PuTTY 或其他串口终端工具。

## 自定义开发

### 修改键盘行为
编辑 `src/usb/hid.rs` 中的 `run_keyboard` 函数：

```rust
pub async fn run_keyboard<'d, D: Driver<'d>>(
    mut keyboard: HidWriter<'d, D, 8>,
) {
    loop {
        // 修改延迟时间
        Timer::after_secs(5).await;
        
        // 修改发送的键码
        let report = [0, 0, 0x0B, 0, 0, 0, 0, 0]; // 0x0B = 'H'
        keyboard.write(&report).await;
        
        // 释放按键
        Timer::after_millis(50).await;
        let release = [0, 0, 0, 0, 0, 0, 0, 0];
        keyboard.write(&release).await;
    }
}
```

### 修改鼠标行为
编辑 `src/usb/hid.rs` 中的 `run_mouse` 函数：

```rust
pub async fn run_mouse<'d, D: Driver<'d>>(
    mut mouse: HidWriter<'d, D, 4>,
) {
    loop {
        Timer::after_secs(3).await;
        
        // 修改移动距离 (x, y)
        let report = [0, 10, 10, 0]; // buttons, x, y, wheel
        mouse.write(&report).await;
    }
}
```

### 修改串口行为
编辑 `src/usb/serial.rs` 中的 `run_cdc_acm` 函数来实现自定义逻辑。

## 依赖项

项目使用以下主要依赖：
- `embassy-usb` v0.3.0 - USB 设备栈
- `embassy-rp` v0.2.0 - RP2040 HAL
- `usbd-hid` v0.8.2 - HID 描述符生成

## 注意事项

1. USB 功能在 Core 0 上运行，确保不与其他任务冲突
2. USB 设备任务使用异步执行器，所有 USB 类共享同一个 USB 总线
3. 当前实现为演示目的，实际应用中可根据需求修改
4. MSC 功能需要等待 embassy-usb 未来版本支持

## 故障排除

### 设备未被识别
- 检查 USB 连接
- 确认 VID/PID 配置
- 查看 defmt 日志输出

### 串口无法通信
- 确认波特率设置
- 检查设备权限 (Linux 下可能需要 sudo 或添加到 dialout 组)

### HID 设备不工作
- 确认主机驱动已加载
- 检查 HID 报告描述符是否正确

## 许可证

本项目遵循原项目的许可证。

