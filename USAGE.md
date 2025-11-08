# USB 功能使用说明

## 概述

本项目支持两种 USB 模式：
1. **HID 键盘模式（默认）**：仅启用 USB HID 键盘功能
2. **复合设备模式**：HID 键盘 + CDC-ACM 串口

## 编译方式

### 方式一：使用 Makefile (推荐)

```bash
# 查看所有可用的 make 目标
make help

# 编译 HID 键盘模式（默认）
make build

# 编译 HID + CDC-ACM 串口模式
make build-serial

# 编译 debug 版本
make build-debug

# 清理构建文件
make clean
```

### 方式二：使用 cargo 直接编译

```bash
# 仅 HID 键盘模式
cargo build --release

# HID + CDC-ACM 串口模式
cargo build --release --features usb-serial

# Debug 模式
cargo build
```

## 为什么默认不启用串口？

1. **体积更小**：不包含 CDC-ACM 代码，编译后的固件更小
2. **更稳定**：单一 HID 设备更简单，兼容性更好
3. **调试用途**：串口功能主要用于调试，生产环境通常不需要

## Windows 错误 10 问题说明与解决方案

如果你在 Windows 上遇到 "错误 10：该设备无法启动" 的问题，这通常是由于：

1. **复合设备描述符问题**：Windows 对 USB 复合设备的描述符要求比较严格
2. **IAD 配置**：需要正确配置接口关联描述符（Interface Association Descriptor）
3. **驱动缓存问题**：Windows 可能缓存了旧的设备信息

### 已实施的改进（2025-11-08）

本项目已经实施以下改进来提高 Windows 兼容性：

✅ **使用不同的 USB PID**
- HID 模式: VID:0x2E8A PID:**0x000A**
- CDC+HID 模式: VID:0x2E8A PID:**0x000C**
- 这避免了 Windows 驱动缓存冲突

✅ **正确的 USB 描述符**
- Device Class: 0xEF (Miscellaneous)
- Device SubClass: 0x02 (Common Class)  
- Device Protocol: 0x01 (IAD)
- 使用 USB 2.0（避免 USB 2.1 的额外要求）

✅ **正确的接口顺序**
- Interface 0: CDC Communication
- Interface 1: CDC Data
- Interface 2: HID Keyboard

### 快速修复步骤

如果仍然遇到问题：

1. **清除 Windows USB 驱动缓存**
   - 在设备管理器中卸载问题设备
   - **勾选**"删除此设备的驱动程序软件"
   - 拔出 Pico，等待 5 秒后重新插入

2. **重新编译和烧录**
   ```bash
   make clean
   make build-serial  # 使用最新的 USB 配置
   ```

3. **使用不同的 USB 端口**
   - 尝试主板后置 USB 端口
   - 避免使用 USB 集线器

4. **查看详细的故障排除指南**
   - 参考 `WINDOWS_TROUBLESHOOTING.md` 获取详细步骤

### 替代方案

如果问题持续：
- 使用仅 HID 模式（默认编译）：`make build`
- 通过 UART0 (GPIO0/1) 查看日志
- 在 Linux 系统下测试（通常没有问题）

## USB 配置对比

### HID 模式（默认）

```
设备描述符:
  - VID: 0x2E8A (Raspberry Pi)
  - PID: 0x000A  ← HID 专用
  - Product: "RP1 HID Keyboard"
  - Device Class: 0x00 (由接口定义)
  
接口:
  - Interface 0: HID Keyboard
```

### 复合设备模式（usb-serial feature）

```
设备描述符:
  - VID: 0x2E8A (Raspberry Pi)
  - PID: 0x000C  ← CDC+HID 专用（避免驱动冲突）
  - Product: "RP1 Serial + Keyboard"
  - Device Class: 0xEF (Miscellaneous)
  - Device SubClass: 0x02 (Common Class)
  - Device Protocol: 0x01 (IAD)
  
接口:
  - Interface 0: CDC Communication Interface (控制)
  - Interface 1: CDC Data Interface (数据)
  - Interface 2: HID Keyboard
  
注意：使用不同的 PID 可以避免 Windows 驱动缓存问题
```

## 使用串口功能

如果启用了 `usb-serial` feature，设备会在电脑上显示为：
- **HID 键盘设备**：自动识别，无需驱动
- **CDC-ACM 串口**：
  - Windows: 显示为 COMx 端口（如 COM8）
    - 设备名称："RP1 Serial + Keyboard" (VID:2E8A PID:000C)
  - Linux: 显示为 /dev/ttyACMx
  - macOS: 显示为 /dev/cu.usbmodemXXX

**重要提示**：Windows 用户首次使用时，请清除旧的 USB 驱动缓存！

### 串口测试

```bash
# Linux/macOS
screen /dev/ttyACM0 115200

# Windows (使用串口工具，如 PuTTY)
# 连接到对应的 COM 端口，波特率 115200

# 测试回显
# 输入任何字符，设备会将其回显
```

## 硬件连接

### UART0（用于日志输出）
- TX: GPIO0
- RX: GPIO1  
- 波特率: 115200

可以通过 UART-USB 转换器连接到电脑查看日志：

```bash
# Linux/macOS
screen /dev/ttyUSB0 115200

# Windows (使用串口工具)
# 连接到对应的 COM 端口
```

## 烧录固件

1. **按住 BOOTSEL 按钮**，然后将 Pico 插入电脑
2. Pico 会显示为 USB 存储设备（RPI-RP2）
3. 将生成的 `rp1-embassy.uf2` 文件复制到该磁盘
4. 固件会自动烧录并重启

## 故障排除

### USB 设备无法识别

1. 检查 USB 线缆是否支持数据传输
2. 尝试不同的 USB 端口
3. 查看设备管理器（Windows）或 dmesg（Linux）的错误信息
4. 尝试使用仅 HID 模式（默认编译）

### CDC-ACM 串口无法打开

1. 确认使用 `--features usb-serial` 编译
2. Windows 用户检查设备管理器中是否有感叹号
3. 尝试更新或重新安装 USB 驱动
4. 如果问题持续，使用 UART0 进行调试输出

### 编译错误

```bash
# 清理并重新编译
cargo clean
cargo build --release

# 或使用 Makefile
make clean
make build
```

## 开发建议

1. **日常开发**：使用默认 HID 模式，通过 UART0 查看日志
2. **深度调试**：启用 `usb-serial` feature，使用 USB 串口
3. **生产部署**：使用默认 HID 模式，减小固件体积

## 相关资源

- [Embassy 文档](https://embassy.dev/)
- [RP2040 数据手册](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
- [USB HID 规范](https://www.usb.org/hid)
- [USB CDC-ACM 规范](https://www.usb.org/document-library/class-definitions-communication-devices-12)

