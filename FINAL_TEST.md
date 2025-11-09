# 最终测试方案 - CDC + HID 双工作

## 🎯 问题回顾

- ✅ Device Class 0xEF: HID 工作，CDC 不工作（Windows 错误 10）
- ✅ Device Class 0x02: CDC 工作，HID 不工作（被识别为纯 CDC 设备）
- 🔥 Device Class 0x00: **两者都应该工作！**

## 💡 解决方案

**使用 Device Class 0x00（由接口定义）**

```
Device Descriptor:
  bDeviceClass: 0x00    ← 不在设备级定义类型
  bDeviceSubClass: 0x00
  bDeviceProtocol: 0x00

Interface Descriptors:
  Interface 0: CDC Communication (with IAD)  ← 接口自己声明
  Interface 1: CDC Data                      ← 接口自己声明
  Interface 2: HID Keyboard                  ← 接口自己声明
```

这样 Windows 会：
- 为接口 0-1（CDC，通过 IAD 关联）加载 usbser.sys
- 为接口 2（HID）加载 HID 驱动
- 两个功能独立工作！

## 🧪 测试步骤

### 步骤 1: 清除所有旧驱动（关键！）

```
设备管理器
└─ 查看 → 显示隐藏的设备
└─ 找到所有 VID:2E8A 的设备（包括隐藏的）
   - PID:000A 的旧 HID 设备
   - PID:000C 的旧复合设备
   - 任何带感叹号的设备
└─ 逐个卸载
   ✅ 勾选 "删除此设备的驱动程序软件"
└─ 拔出 Pico，等待 30 秒
└─ 🔥 重启 Windows（强烈推荐！）
```

### 步骤 2: 烧录新固件

```bash
# 当前目录的 rp1-embassy.uf2 已是最新版本
# Device Class: 0x00 (由接口定义)

# 或重新编译
make clean
make build-serial
```

烧录：
1. 按住 BOOTSEL，插入 Pico
2. 复制 rp1-embassy.uf2 到 RPI-RP2
3. 等待重启

### 步骤 3: 验证两个功能

打开设备管理器，应该同时看到：

**✅ 成功的情况：**

```
通用串行总线控制器
  └─ USB 复合设备 (可能显示为 RP1 Serial + Keyboard)

端口 (COM 和 LPT)
  └─ USB Serial Device (COMx) ✅ 无感叹号
      [VID_2E8A&PID_000C&MI_00]
      驱动: usbser.sys

人体学输入设备
  └─ HID-compliant device ✅ 无感叹号
      [VID_2E8A&PID_000C&MI_02]
      驱动: HID Class Driver
```

### 步骤 4: 功能测试

**测试 CDC 串口：**
```bash
# 使用 PuTTY, Tera Term, 或 PowerShell
# 连接到 COMx, 115200 波特率
# 输入文本，应该看到回显
```

```powershell
# PowerShell 测试
$port = new-Object System.IO.Ports.SerialPort COM8,115200
$port.Open()
$port.WriteLine("Hello CDC")
$received = $port.ReadLine()
Write-Host "Received: $received"  # 应该显示 "Hello CDC"
$port.Close()
```

**测试 HID 键盘：**
- HID 键盘应该自动被识别
- 设备管理器中无感叹号
- （当前代码每 5 秒发送一次虚拟按键，可以解开注释测试）

## 🔍 诊断

### 如果 CDC 不工作但 HID 工作

说明回到了原始状态。可能是：
- 旧固件没有完全替换
- Windows 缓存问题
- 重新清除驱动并重启

### 如果 HID 不工作但 CDC 工作

说明烧录了错误的固件（Device Class 0x02）。
- 确认使用最新的 rp1-embassy.uf2
- 检查编译时间戳

### 如果两者都不工作

检查 USB 描述符：

```powershell
# 查看设备信息
Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_2E8A&PID_000C*"} | Format-List *

# 检查是否有错误
Get-PnpDevice | Where-Object {$_.Status -eq "Error"}
```

## 📊 Device Class 对比

| Device Class | 效果 | CDC | HID | 说明 |
|--------------|------|-----|-----|------|
| 0x02 (CDC) | 纯 CDC | ✅ | ❌ | 整个设备被视为 CDC，HID 被忽略 |
| 0xEF (Misc+IAD) | 标准复合 | ❌ | ✅ | 某些 Windows 版本对 CDC 支持不好 |
| 0x00 (Interface) | 接口定义 | ✅ | ✅ | **推荐！** 每个接口独立识别 |

## 🎓 技术解释

### 为什么 0x00 可以工作？

**Device Class 0x00** 告诉 Windows：
> "不要在设备级判断我的类型，看我的每个接口！"

然后：
1. **Interface 0-1 (CDC)** 通过 IAD 关联，声明为 CDC
   - Windows 看到 IAD，知道这两个接口组成一个 CDC 功能
   - 加载 usbser.sys 驱动
   - 创建 COM 端口

2. **Interface 2 (HID)** 独立声明为 HID
   - Windows 看到 HID 接口描述符
   - 加载 HID Class Driver
   - 识别为键盘设备

3. **两者不冲突** 因为在接口级别分别处理

### embassy-usb 的 IAD 处理

embassy-usb 会自动为 CDC-ACM 添加 IAD：
```
Interface Association Descriptor (IAD):
  bFirstInterface: 0        ← CDC 通信接口
  bInterfaceCount: 2        ← CDC 占用 2 个接口
  bFunctionClass: 0x02      ← CDC 类
  bFunctionSubClass: 0x02   ← ACM 子类
```

即使 Device Class 是 0x00，这个 IAD 也会生成，确保 Windows 正确识别 CDC 功能。

## 📋 检查清单

测试前确认：
- [ ] 已卸载所有 VID:2E8A 的旧设备
- [ ] 已勾选"删除驱动程序软件"
- [ ] 已重启 Windows
- [ ] 使用最新的 rp1-embassy.uf2 (Device Class 0x00)
- [ ] USB 线直连主板端口（不用集线器）

测试成功标志：
- [ ] 设备管理器中看到 COM 端口，无感叹号
- [ ] 设备管理器中看到 HID 设备，无感叹号
- [ ] 串口可以打开并回显数据
- [ ] HID 设备被正确识别

## 🚀 如果成功

恭喜！你现在有一个完全工作的 CDC + HID 复合设备。

**日常使用建议：**
- 开发/调试：使用此版本（CDC + HID）
- 生产/发布：使用 HID 模式（`make build`）更稳定

**固件信息：**
- VID: 0x2E8A (Raspberry Pi)
- PID: 0x000C (CDC + HID)
- Device Class: 0x00 (Interface Defined)
- 接口: CDC (0,1) + HID (2)

## ⚠️ 如果仍然失败

请提供以下信息：

1. **Windows 版本和构建号**
```powershell
Get-ComputerInfo | Select WindowsVersion,WindowsBuildLabEx
```

2. **设备状态**
```powershell
Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_2E8A*"} | Format-List
```

3. **USB 描述符**（从 Linux/macOS）
```bash
lsusb -v -d 2e8a:000c
```

4. **设备管理器截图**
- 显示 COM 端口和 HID 设备的状态
- 如有错误，显示错误详情

---

**版本**: v4 (2025-11-08)  
**Device Class**: 0x00 (Interface Defined)  
**预期结果**: CDC ✅ + HID ✅  
**测试状态**: 等待反馈 🎯

