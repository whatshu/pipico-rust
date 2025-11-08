# Windows 串口兼容性故障排除指南

## 问题概述

Windows 错误 10（"该设备无法启动"）通常出现在 USB CDC-ACM 复合设备上，主要原因包括：

1. USB 描述符配置问题
2. 驱动程序不匹配
3. Windows USB 驱动缓存问题
4. USB 端口电源管理问题

## 最新改进（2025-11-08）

本项目已经实施了以下改进来提高 Windows 兼容性：

### ✅ 使用不同的 PID
- **HID 模式**: VID:0x2E8A PID:0x000A
- **CDC+HID 模式**: VID:0x2E8A PID:0x000C

这避免了 Windows 驱动缓存冲突。

### ✅ 正确的设备类配置
- Device Class: 0xEF (Miscellaneous Device)
- Device SubClass: 0x02 (Common Class)
- Device Protocol: 0x01 (IAD)

### ✅ 使用 USB 2.0
- 使用 USB 2.0 而非 USB 2.1
- 避免了 Windows 对 USB 2.1 设备的额外描述符要求

### ✅ 接口顺序
- Interface 0: CDC Communication
- Interface 1: CDC Data
- Interface 2: HID Keyboard

## 故障排除步骤

### 步骤 1: 清除 Windows USB 驱动缓存

Windows 会缓存 USB 设备的驱动信息，有时会导致问题。

**方法 A: 使用设备管理器**

1. 打开"设备管理器"
2. 找到问题设备（可能在"其他设备"或"端口"下）
3. 右键点击 → "卸载设备"
4. **勾选**"删除此设备的驱动程序软件"
5. 点击"卸载"
6. 拔出 Pico，等待 5 秒
7. 重新插入 Pico

**方法 B: 使用管理员命令提示符**

```cmd
:: 以管理员身份运行
pnputil /enum-devices /class USB
pnputil /delete-driver oem<number>.inf /uninstall

:: 或者清除所有 USB 驱动缓存
rundll32.exe devmgr.dll DeviceManager_Execute
```

### 步骤 2: 重新编译固件

确保使用最新的代码编译：

```bash
# 清理旧的构建
make clean

# 重新编译串口版本
make build-serial

# 或使用 cargo
cargo clean
cargo build --release --features usb-serial
```

### 步骤 3: 烧录并测试

1. 按住 BOOTSEL，插入 Pico
2. 复制新的 `rp1-embassy.uf2` 到 RPI-RP2
3. 设备会自动重启
4. 在设备管理器中检查设备状态

### 步骤 4: 检查设备管理器

打开"设备管理器"，查找：

**成功的情况：**
- "端口 (COM 和 LPT)" → "USB Serial Device (COMx)" ✓
- "人体学输入设备" → "HID Keyboard Device" ✓

**失败的情况：**
- "其他设备" → "Composite Device" 或 "Unknown Device" ✗
- 设备上有黄色感叹号 ✗

### 步骤 5: 手动安装驱动（如果需要）

如果自动安装失败，可以尝试手动指定驱动：

1. 右键点击问题设备 → "更新驱动程序"
2. 选择"浏览我的计算机以查找驱动程序"
3. 选择"让我从计算机上的可用驱动程序列表中选取"
4. 选择"端口 (COM 和 LPT)"
5. 制造商选择"Microsoft"
6. 型号选择"USB Serial Device"或"USB CDC Serial"
7. 点击"下一步"

## 高级诊断

### 查看详细的设备信息

使用 USBDeview 或 USB Device Tree Viewer：

1. 下载 [USBDeview](https://www.nirsoft.net/utils/usb_devices_view.html)
2. 找到 VID:2E8A PID:000C 的设备
3. 查看设备描述符：
   - bcdUSB: 应该是 0x0200 (USB 2.0)
   - bDeviceClass: 应该是 0xEF
   - bDeviceSubClass: 应该是 0x02
   - bDeviceProtocol: 应该是 0x01

### 查看 Windows 设备安装日志

```powershell
# 以管理员身份运行 PowerShell
Get-WinEvent -LogName "Microsoft-Windows-DriverFrameworks-UserMode/Operational" | 
    Where-Object {$_.TimeCreated -gt (Get-Date).AddHours(-1)} | 
    Select-Object TimeCreated, Message
```

### 使用 Windows USB 测试工具

1. 下载 [USBSTP (USB Sniffer and Protocol Decoder)](https://www.hhdsoftware.com/free-usb-monitor)
2. 连接设备并查看 USB 枚举过程
3. 检查是否有描述符错误

## 常见错误和解决方案

### 错误: "该设备无法启动 (代码 10)"

**原因：**
- USB 描述符配置问题
- 驱动程序冲突
- Windows 驱动缓存问题

**解决方案：**
1. 使用最新的固件（已修复 USB 描述符）
2. 清除 USB 驱动缓存（见步骤 1）
3. 尝试不同的 USB 端口
4. 重启 Windows

### 错误: "指定不存在的设备"

**原因：**
- USB 设备枚举失败
- 接口描述符不完整

**解决方案：**
1. 确保使用 `--features usb-serial` 编译
2. 检查 USB 线缆质量
3. 尝试直接连接到主板 USB 端口（不使用集线器）

### 错误: "找不到该设备的驱动程序"

**原因：**
- Windows 没有识别为标准 CDC-ACM 设备

**解决方案：**
1. 手动安装驱动（见步骤 5）
2. 使用 Zadig 工具安装 WinUSB 驱动（不推荐，会影响 HID 功能）

### HID 键盘工作，但串口不工作

**解决方案：**
1. 确认使用的是串口版本固件：`make build-serial`
2. 在设备管理器中检查是否有 COM 端口
3. 尝试在设备管理器中"扫描硬件改动"

## 替代方案

如果 Windows 串口问题持续存在，考虑以下替代方案：

### 方案 1: 仅使用 HID 模式（推荐）

```bash
make build  # 不带 --features usb-serial
```

通过 UART0 (GPIO0/1) 连接 USB-TTL 转换器查看日志。

### 方案 2: 使用 Linux 虚拟机

在 Linux 下，CDC-ACM 驱动支持更好：

```bash
# 在 Linux 虚拟机中
lsusb | grep 2e8a
ls /dev/ttyACM*
screen /dev/ttyACM0 115200
```

### 方案 3: 使用 WSL2 + USB/IP

Windows 11 支持 WSL2 USB 连接：

```bash
# Windows PowerShell (管理员)
usbipd wsl list
usbipd wsl attach --busid <busid>

# WSL2 中
ls /dev/ttyACM*
```

## 测试清单

- [ ] 使用最新代码编译：`make build-serial`
- [ ] 清除 Windows USB 驱动缓存
- [ ] 烧录新固件
- [ ] 在设备管理器中检查设备
- [ ] 检查是否有 COM 端口
- [ ] 尝试打开 COM 端口（PuTTY, Tera Term 等）
- [ ] 测试回显功能（输入字符应该被回显）

## 成功案例

设备正常工作时，你应该看到：

**设备管理器：**
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8)  [正常，无感叹号]

人体学输入设备
  └─ HID-compliant device
```

**串口测试：**
```bash
# 使用 PuTTY 或其他串口工具
# 连接到 COM8, 115200 波特率
# 输入: Hello
# 输出: Hello  (回显)
```

## 其他资源

- [USB CDC 规范](https://www.usb.org/document-library/class-definitions-communication-devices-12)
- [Windows USB 驱动开发文档](https://docs.microsoft.com/en-us/windows-hardware/drivers/usbcon/)
- [Embassy USB 文档](https://embassy.dev/)

## 报告问题

如果问题仍然存在，请提供以下信息：

1. Windows 版本（Win10/Win11）
2. 设备管理器截图
3. USBDeview 中的设备信息
4. 编译命令和固件版本
5. 错误代码和错误信息
6. 设备安装日志（如果可能）

## 联系和支持

如果以上方法都无法解决问题，建议：

1. 使用默认 HID 模式（更稳定）
2. 通过 UART0 查看日志
3. 在 Linux 系统下测试（通常没有问题）

