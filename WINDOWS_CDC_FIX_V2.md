# Windows CDC-ACM 错误 10 修复 v2

## 🔥 重要更新 (2025-11-08 v3)

经过深入分析和调研，我们发现问题的根本原因：

### Windows 对设备类的敏感性

Windows 对 USB 设备类（Device Class）的处理非常严格。之前的配置使用：
```
Device Class: 0xEF (Miscellaneous Device)
```

但某些 Windows 版本对这种配置的支持不够完善，特别是在复合设备（CDC + HID）中。

### 新的修复方案

**已修改为：**
```
Device Class: 0x02 (Communications Device Class - CDC)
Device SubClass: 0x00
Device Protocol: 0x00
```

这种配置更直接地表明设备的主要功能是 CDC（串口），Windows 的兼容性通常更好。

## 测试新固件

### 步骤 1: 彻底清除旧设备

**重要！** 必须完全清除旧的 USB 驱动信息：

1. **打开设备管理器**
2. **查看 → 显示隐藏的设备**
3. **卸载所有相关设备**：
   - 找到所有 VID:2E8A 的设备（包括隐藏的）
   - 右键 → 卸载设备
   - ✅ **勾选** "删除此设备的驱动程序软件"
   - 卸载所有找到的相关设备

4. **清除驱动缓存**（以管理员身份运行 PowerShell）：
```powershell
# 列出所有 USB 驱动
pnputil /enum-drivers

# 找到包含 2E8A 或相关的驱动，删除它们
# 例如：pnputil /delete-driver oem123.inf /uninstall
```

5. **拔出 Pico，等待 30 秒**

6. **重启 Windows**（强烈建议！）

### 步骤 2: 烧录新固件

```bash
# 当前目录的 rp1-embassy.uf2 已经是最新版本
# 使用新的 Device Class 0x02 配置

# 或重新编译
make clean
make build-serial
```

烧录：
1. 按住 BOOTSEL，插入 Pico
2. 复制 `rp1-embassy.uf2` 到 RPI-RP2
3. 等待设备重启

### 步骤 3: 验证

打开设备管理器，应该看到：

**成功的情况：**
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COMx) ✅ 无感叹号

人体学输入设备
  └─ HID-compliant device ✅
```

**设备属性应该显示：**
- 名称: "RP1 Serial + Keyboard"
- VID: 2E8A
- PID: 000C
- Device Class: 02 (CDC)
- 驱动: usbser.sys

## 如果仍然失败

### 方案 A: 仅 CDC 模式（排除 HID 干扰）

我们可以创建一个仅 CDC 的版本来测试：

修改 `src/main.rs` line 157-165，注释掉 HID 创建：

```rust
// 暂时禁用 HID，仅测试 CDC
// static HID_HANDLER: StaticCell<usb::hid::HidRequestHandler> = StaticCell::new();
// static KEYBOARD_STATE: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();
// 
// let hid_handler = HID_HANDLER.init(usb::hid::HidRequestHandler {});
// let keyboard_state = KEYBOARD_STATE.init(embassy_usb::class::hid::State::new());
// 
// let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);
// log_info_sync!("USB", "HID keyboard created");
```

同时修改 line 180-187，只运行 CDC：

```rust
let usb_fut = usb_device.run();
// let keyboard_fut = usb::hid::run_keyboard(keyboard);

#[cfg(feature = "usb-serial")]
{
    let cdc_fut = usb::serial::run_cdc_acm(cdc_acm);
    log_info_sync!("USB", "CDC-ACM task starting...");
    embassy_futures::join::join(usb_fut, cdc_fut).await;
}
```

重新编译并测试。如果仅 CDC 可以工作，说明问题是复合设备配置引起的。

### 方案 B: 尝试 IAD 配置（需要编辑代码）

如果方案 A（Device Class 0x02）仍然失败，可以尝试回到 IAD 配置。

修改 `src/usb/mod.rs` line 50-54：

```rust
// 注释掉 CDC 配置
// config.device_class = 0x02;
// config.device_sub_class = 0x00;
// config.device_protocol = 0x00;

// 启用 IAD 配置
config.device_class = 0xEF;    // Miscellaneous Device
config.device_sub_class = 0x02; // Common Class
config.device_protocol = 0x01;  // Interface Association Descriptor
```

### 方案 C: 使用 Zadig 安装 WinUSB 驱动

⚠️ 这会改变设备的驱动模型，不推荐，但可以作为最后手段：

1. 下载 Zadig: https://zadig.akeo.ie/
2. 运行 Zadig
3. Options → List All Devices
4. 选择您的 CDC 接口
5. 安装 WinUSB 驱动

**缺点：** 这会使设备无法作为标准 COM 端口使用，需要特殊的应用程序访问。

## 诊断信息收集

如果以上方案都失败，请收集以下信息：

### 1. USB 描述符（从 Linux/macOS）

```bash
# Linux
lsusb -v -d 2e8a:000c > usb_descriptor_linux.txt
dmesg | tail -50 > dmesg_output.txt

# macOS
system_profiler SPUSBDataType > usb_descriptor_mac.txt
```

### 2. Windows 诊断信息

```powershell
# 设备信息
Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_2E8A*"} | Format-List * > device_info.txt

# USB 事件日志
Get-WinEvent -LogName "System" -MaxEvents 100 | 
    Where-Object {$_.Message -like "*USB*" -or $_.Message -like "*2E8A*"} | 
    Format-List TimeCreated,Message > usb_events.txt

# Windows 版本
Get-ComputerInfo | Select-Object WindowsVersion,WindowsBuildLabEx,OsHardwareAbstractionLayer > windows_info.txt
```

### 3. USBView 截图

1. 下载 USBView (Windows SDK 或 Microsoft 网站)
2. 找到您的设备
3. 截图设备描述符页面
4. 特别注意：
   - Device Descriptor 部分
   - Configuration Descriptor 部分
   - Interface Descriptors 部分
   - IAD（如果有）

### 4. 设备管理器截图

- 设备属性的所有标签页
- 特别是"详细信息"标签页中的：
  - 硬件 ID
  - 兼容 ID
  - 设备实例路径
  - 驱动程序关键字

## 技术背景

### 为什么改用 Device Class 0x02？

1. **更直接的识别**：当设备主要功能是 CDC 时，使用 0x02 更明确
2. **更好的向后兼容性**：一些旧版 Windows 对 0x02 的支持更好
3. **标准驱动加载**：Windows 会更直接地加载 usbser.sys 驱动

### Device Class 对比

| Class | 值 | 适用场景 | Windows 支持 |
|-------|-----|---------|--------------|
| CDC | 0x02 | 单纯 CDC 或 CDC 为主 | ✅ XP+ |
| Miscellaneous | 0xEF | 标准复合设备（需 IAD） | ✅ Vista SP2+ |
| Vendor Specific | 0xFF | 自定义设备 | 需要自定义驱动 |

### embassy-usb 的 IAD 处理

Embassy-usb 会自动为 CDC-ACM 类添加 IAD（Interface Association Descriptor），这确保了多接口设备能正确分组。关键是设备级的 Class 配置必须与 Windows 期望匹配。

## 已知问题和限制

### Windows 版本差异

- **Windows 10/11**: 通常兼容性最好，支持标准 CDC-ACM
- **Windows 8/8.1**: 可能需要特殊配置
- **Windows 7**: 支持 CDC，但某些更新可能影响兼容性
- **Windows XP**: 不推荐，支持有限

### USB 端口类型

- **USB 2.0 端口**: 推荐
- **USB 3.0/3.1 端口**: 通常向后兼容，但某些主板实现可能有问题
- **USB-C 端口**: 应该工作，但需要好的线缆
- **USB 集线器**: 可能引入兼容性问题，建议直连主板端口

## 成功案例配置

如果新固件工作正常，您应该看到：

**设备管理器：**
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8)
      [VID_2E8A&PID_000C&MI_00]
      驱动程序: usbser.sys
      设备类: CDC (02)
      ✅ 无错误

人体学输入设备  
  └─ HID-compliant device
      [VID_2E8A&PID_000C&MI_02]
      ✅ 无错误
```

**串口测试成功：**
```bash
# PowerShell 测试
$port = new-Object System.IO.Ports.SerialPort COM8,115200
$port.Open()
$port.WriteLine("Hello")
$received = $port.ReadLine()
Write-Host "Received: $received"  # 应该显示 "Hello"
$port.Close()
```

## 下一步

1. **测试新固件** - 使用新的 Device Class 0x02 配置
2. **完全清除旧驱动** - 不要跳过这一步！
3. **重启 Windows** - 确保驱动缓存清空
4. **收集诊断信息** - 如果仍然失败
5. **尝试备选方案** - 仅 CDC 模式或 IAD 配置

## 参考资源

- [USB CDC Class 规范](https://www.usb.org/document-library/class-definitions-communication-devices-12)
- [Windows USB 驱动开发](https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/)
- [embassy-usb 文档](https://docs.embassy.dev/embassy-usb/)
- [Microsoft USB 故障排除](https://support.microsoft.com/en-us/windows/fix-usb-problems-in-windows-4-06-db07-a07e-a3c7-03a0)

---

**更新日期**: 2025-11-08 v3  
**关键修改**: Device Class 从 0xEF 改为 0x02  
**目标**: 提高 Windows CDC-ACM 兼容性  
**状态**: 等待测试反馈 🔄

