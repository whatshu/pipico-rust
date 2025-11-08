# Windows CDC-ACM 测试指南

## 问题修复总结

已针对 Windows CDC-ACM "错误 10" 问题实施以下修复：

### ✅ 主要改进

1. **独立的 USB PID**
   - HID 模式: `VID:0x2E8A` `PID:0x000A`
   - CDC+HID 模式: `VID:0x2E8A` `PID:0x000C` ← **新的 PID**
   - 避免 Windows 驱动缓存冲突

2. **更清晰的产品名称**
   - HID: "RP1 HID Keyboard"
   - CDC+HID: "RP1 Serial + Keyboard"

3. **正确的 USB 描述符**
   - Device Class: 0xEF (Miscellaneous)
   - Device SubClass: 0x02 (Common)
   - Device Protocol: 0x01 (IAD)
   - USB 2.0 规范

## 测试步骤

### 步骤 1: 清除旧的 Windows 驱动缓存 ⚠️ 重要！

这是解决"错误 10"最关键的步骤！

1. 打开"设备管理器"（Win+X → 设备管理器）
2. 找到旧的设备（可能显示为）：
   - "Composite Device" 带感叹号
   - "未知设备"
   - 或在"端口"下的旧 COM 设备
3. 右键点击 → "卸载设备"
4. **务必勾选**"删除此设备的驱动程序软件"
5. 点击"卸载"
6. 拔出 Pico
7. **等待 10 秒**（让 Windows 完全清除缓存）

### 步骤 2: 烧录新固件

```bash
# 方法 A: 使用已生成的固件
# 当前目录下的 rp1-embassy.uf2 已经是最新版本

# 方法 B: 重新编译
make clean
make build-serial
```

烧录：
1. 按住 Pico 上的 **BOOTSEL** 按钮
2. 插入 USB 线
3. 将 `rp1-embassy.uf2` 复制到 RPI-RP2 磁盘
4. 设备会自动重启

### 步骤 3: 验证设备

打开设备管理器，应该看到：

**✓ 成功的情况：**
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8)  ← 无感叹号
      [设备实例路径包含 VID_2E8A&PID_000C]

人体学输入设备
  └─ HID-compliant device
```

**✗ 如果还是失败：**
- 确认已经勾选了"删除驱动程序软件"
- 尝试重启 Windows
- 查看 `WINDOWS_TROUBLESHOOTING.md` 获取更多方法

### 步骤 4: 测试串口功能

使用串口工具（PuTTY、Tera Term、Arduino Serial Monitor 等）：

1. 连接到新的 COM 端口（如 COM8）
2. 设置波特率：115200
3. 打开串口
4. 输入任意文本（如 `Hello`）
5. 应该能看到回显：`Hello`

**PowerShell 测试：**
```powershell
# 查看设备
Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_2E8A&PID_000C*"}

# 列出 COM 端口
[System.IO.Ports.SerialPort]::getportnames()
```

### 步骤 5: 测试 HID 键盘

HID 键盘应该自动工作，无需驱动程序。

## 预期结果

### 设备管理器中的设备信息

**USB 复合设备：**
- 名称: "RP1 Serial + Keyboard"
- VID: 0x2E8A
- PID: 0x000C
- 制造商: "RP1 Embassy"

**COM 端口：**
- 名称: "USB Serial Device (COMx)"
- 驱动程序: usbser.sys (Windows 自带)

**HID 设备：**
- 名称: "HID-compliant device"
- 类型: Keyboard

## 常见问题

### Q1: 设备管理器中看不到 COM 端口？

**检查项：**
- [ ] 确认使用的是 `build-serial` 编译的固件
- [ ] 在"通用串行总线控制器"中查找"USB 复合设备"
- [ ] 右键设备 → "属性" → "硬件 ID"，确认 PID 是 000C

**解决方案：**
```bash
# 重新编译确认
make clean
make build-serial  # 不是 make build
```

### Q2: 仍然显示"错误 10"？

**可能的原因：**
1. 没有完全清除旧驱动
2. 使用了旧的固件（PID 仍是 000A）
3. Windows USB 驱动损坏

**解决方案：**
```powershell
# 以管理员身份运行 PowerShell
# 查看所有 USB 设备
pnputil /enum-devices /class USB

# 删除所有 2E8A 设备的驱动
pnputil /delete-driver oem*.inf /uninstall
```

然后重启 Windows，再重新插入设备。

### Q3: COM 端口无法打开？

**检查项：**
- [ ] 确认 COM 端口没有被其他程序占用
- [ ] 尝试不同的串口工具
- [ ] 检查 Windows 防火墙设置

**测试命令：**
```powershell
# PowerShell 测试
$port = new-Object System.IO.Ports.SerialPort COM8,115200,None,8,one
$port.Open()
$port.WriteLine("Hello")
$received = $port.ReadLine()
Write-Host "Received: $received"
$port.Close()
```

### Q4: 两个模式切换后混乱？

这正是我们使用不同 PID 的原因！

**解决方案：**
- HID 模式 (PID:000A) 和 CDC+HID 模式 (PID:000C) 会被识别为不同设备
- Windows 会为每个 PID 分配独立的驱动
- 切换模式后会显示为不同的设备

## 诊断信息收集

如果问题仍然存在，请收集以下信息：

### 1. 设备信息
```powershell
# 运行并保存输出
Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_2E8A*"} | Format-List *
```

### 2. 驱动程序信息
在设备管理器中：
- 右键设备 → "属性"
- 截图"常规"、"驱动程序"、"详细信息"标签页

### 3. USB 枚举日志
```powershell
Get-WinEvent -LogName "Microsoft-Windows-DriverFrameworks-UserMode/Operational" | 
    Where-Object {$_.TimeCreated -gt (Get-Date).AddMinutes(-10) -and $_.Message -like "*2E8A*"} | 
    Format-List TimeCreated,Message | Out-File usb_log.txt
```

### 4. 固件版本
确认你使用的是最新编译的固件：
```bash
ls -lh rp1-embassy.uf2
# 应该显示最近的时间戳
```

## 成功案例示例

**设备管理器 - 成功的显示：**
```
通用串行总线控制器
  └─ USB 复合设备
      [RP1 Serial + Keyboard]

端口 (COM 和 LPT)
  └─ USB Serial Device (COM8)  ✓

人体学输入设备
  └─ HID-compliant device  ✓
```

**串口测试 - 成功的输出：**
```
Connected to COM8 at 115200 baud

> Hello World
Hello World  ← 回显

> Test 123
Test 123  ← 回显
```

## 下一步

测试成功后：

1. **日常使用建议**
   - 生产/发布: 使用 HID 模式 (`make build`)
   - 开发/调试: 使用 CDC+HID 模式 (`make build-serial`)

2. **查看完整文档**
   - `README.md` - 项目概览
   - `USAGE.md` - 详细使用说明
   - `WINDOWS_TROUBLESHOOTING.md` - 深度故障排除

3. **报告问题**
   如果修复无效，请提供上述诊断信息

## 技术细节

### 为什么使用不同的 PID？

Windows 会为每个 VID:PID 组合缓存驱动信息。当设备的接口配置改变时（如从 HID 改为 CDC+HID），Windows 可能会尝试使用旧的驱动配置，导致"错误 10"。

使用不同的 PID 让 Windows 将它们视为完全不同的设备，避免了缓存问题。

### USB 描述符详情

**HID 模式 (PID:000A):**
```
Device Descriptor:
  bDeviceClass: 0x00 (Defined at Interface level)
  bDeviceSubClass: 0x00
  bDeviceProtocol: 0x00
  idVendor: 0x2E8A
  idProduct: 0x000A
```

**CDC+HID 模式 (PID:000C):**
```
Device Descriptor:
  bDeviceClass: 0xEF (Miscellaneous)
  bDeviceSubClass: 0x02 (Common Class)
  bDeviceProtocol: 0x01 (Interface Association)
  idVendor: 0x2E8A
  idProduct: 0x000C
  
Interfaces:
  Interface 0: CDC Control (with IAD)
  Interface 1: CDC Data
  Interface 2: HID Keyboard
```

## 联系和反馈

测试结果反馈：
- ✅ 成功：太好了！可以正常使用了
- ⚠️ 部分成功：查看故障排除文档
- ❌ 失败：收集诊断信息并报告

祝测试顺利！🎉

