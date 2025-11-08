# Windows CDC-ACM 深度调试指南

## 问题现状

- ✅ Linux 和 macOS 可以正常识别和使用串口
- ❌ Windows 显示错误 10："该设备无法启动"
- ❌ 设备管理器中显示黄色感叹号

这种情况说明 USB 描述符基本正确（否则 Linux/macOS 也会有问题），但 Windows 对某些配置更敏感。

## 可能的根本原因

### 1. 设备类配置问题 ⚠️ 最可能的原因

Windows 对 USB 设备类（Device Class）的识别非常严格：

**当前配置（方案 A）:**
```
bDeviceClass = 0xEF (Miscellaneous Device)
bDeviceSubClass = 0x02 (Common Class)
bDeviceProtocol = 0x01 (IAD)
```

**替代配置（方案 B）:**
```
bDeviceClass = 0x02 (Communications Device Class)
bDeviceSubClass = 0x00
bDeviceProtocol = 0x00
```

**为什么方案 B 可能更好？**
- 一些老版本 Windows（或某些 Windows 配置）可能对 0x02 (CDC) 类的支持更好
- 0xEF 需要 Windows Vista SP2+ 才能正确支持
- 当 CDC 是主要功能时，使用 0x02 可能更直接

## 测试方案

### 方案 1: 尝试 CDC 作为主设备类

修改 `src/usb/mod.rs`：

```rust
#[cfg(feature = "usb-serial")]
{
    // 使用 CDC 作为主类（可能更兼容某些 Windows 版本）
    config.device_class = 0x02;    // CDC (Communications Device Class)
    config.device_sub_class = 0x00;
    config.device_protocol = 0x00;
}
```

### 方案 2: 禁用 HID，仅使用 CDC

为了排除 HID 干扰，我们可以创建一个仅 CDC 的版本：

```rust
// 暂时注释掉 HID 创建代码
// let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);
```

这可以帮助确定问题是否由复合设备配置引起。

## 诊断步骤

### 步骤 1: 收集详细的 USB 描述符信息

使用 USBView 或 USB Device Tree Viewer（从 Linux/macOS）：

```bash
# 在 Linux 上
lsusb -v -d 2e8a:000c > usb_descriptor.txt

# 在 macOS 上
system_profiler SPUSBDataType > usb_descriptor.txt
```

发送 `usb_descriptor.txt` 文件内容，这将帮助我们看到实际的描述符配置。

### 步骤 2: 检查 Windows 事件日志

```powershell
# 以管理员身份运行 PowerShell
Get-WinEvent -LogName "System" -MaxEvents 50 | 
    Where-Object {$_.ProviderName -like "*USB*" -or $_.Message -like "*USB*" -or $_.Message -like "*2E8A*"} |
    Format-List TimeCreated,Message | Out-File -FilePath usb_events.txt
```

### 步骤 3: 使用 Zadig 查看设备信息

1. 下载 Zadig: https://zadig.akeo.ie/
2. 运行 Zadig，不要安装驱动！
3. 在选项中勾选"List All Devices"
4. 找到您的设备（RP1 Serial + Keyboard）
5. 截图设备信息页面，特别是 VID/PID 和接口信息

### 步骤 4: 检查 Windows 版本和更新

```powershell
# 检查 Windows 版本
Get-ComputerInfo | Select-Object WindowsVersion,OsHardwareAbstractionLayer

# 检查最近的 Windows 更新
Get-HotFix | Sort-Object -Property InstalledOn -Descending | Select-Object -First 5
```

某些 Windows 更新可能影响 USB 驱动的兼容性。

## 修复尝试

### 尝试 1: 修改设备类为 CDC (0x02)

<function_calls>
<invoke name="search_replace">
<parameter name="file_path">/home/whatshu/develop/project/pico/rp1-embassy/src/usb/mod.rs
