# Windows 驱动修复指南

## 🎯 当前状态

✅ **固件正常运行**（从 UART 日志可以确认）
✅ **USB 初始化成功**
✅ **所有功能正常工作**

❌ **Windows 无法识别设备**（驱动问题）

## 📊 设备管理器当前状态

```
其他设备
  ├─ 未知设备 (复合设备)  ← USB 复合设备未识别
  └─ 未知设备 (串口)      ← CDC-ACM 未识别

应该变成：
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8)

人体学输入设备
  ├─ HID-compliant keyboard
  └─ HID-compliant mouse
```

## 🔧 解决方案

### 方案 1：使用 Zadig 安装驱动（推荐）

Zadig 是一个自动安装 Windows USB 驱动的工具。

#### 步骤详解

1. **下载 Zadig**
   - 访问：https://zadig.akeo.ie/
   - 下载最新版本（zadig-2.x.exe）
   - 无需安装，直接运行

2. **运行 Zadig**
   - 右键点击 zadig.exe → 以管理员身份运行
   - ⚠️ 必须以管理员权限运行

3. **配置 Zadig**
   - 点击菜单：Options → List All Devices ☑
   - 这样才能看到所有 USB 设备

4. **找到你的设备**
   在下拉列表中查找：
   - "RP1 Embassy Composite Device"
   - 或者 "Composite Device (Interface 0)"
   - 或者显示 VID:2E8A PID:000A 的设备

5. **选择正确的驱动**
   
   **对于 CDC-ACM (串口)**：
   - 驱动选择框中选择：**usbser (v...)**
   - 这是 Windows 内置的 USB Serial 驱动
   - 点击 "Install Driver" 或 "Replace Driver"

   **对于整个复合设备**：
   - 驱动选择：**Composite Parent (Generic USB Hub)**
   - 或者 **USB Serial (CDC)**

6. **等待安装完成**
   - 看到 "Driver installed successfully" 消息
   - 关闭 Zadig

7. **断开并重新连接 USB**
   - 拔掉 USB
   - 等待 5 秒
   - 重新插入

8. **检查设备管理器**
   - Win+X → 设备管理器
   - 展开 "端口 (COM 和 LPT)"
   - 应该看到新的 COM 口

### 方案 2：手动安装驱动

如果 Zadig 不工作，可以手动安装。

#### 为 CDC-ACM (串口) 安装驱动

1. **打开设备管理器**
   - Win+X → 设备管理器

2. **找到未知设备**
   - 展开 "其他设备"
   - 找到带黄色感叹号的设备

3. **更新驱动程序**
   - 右键点击未知设备
   - 选择 "更新驱动程序"

4. **浏览计算机以查找驱动程序**
   - 选择 "浏览我的电脑以查找驱动程序"

5. **从列表中选择**
   - 选择 "让我从计算机上的可用驱动程序列表中选取"

6. **选择设备类型**
   - 选择 "端口 (COM 和 LPT)"
   - 点击 "下一步"

7. **选择驱动**
   - 制造商：**(标准端口类型)**
   - 型号：**USB Serial Device**
   - 点击 "下一步"

8. **完成安装**
   - 等待安装完成
   - 记下分配的 COM 口号

#### 为 HID 设备安装驱动

HID 设备通常会自动识别，如果没有：

1. 右键点击未知设备
2. 更新驱动程序
3. 从列表中选择
4. 选择 "人体学输入设备"
5. 选择 "HID 兼容设备" 或 "USB 输入设备"

### 方案 3：使用 INF 文件（高级）

如果以上方法都不行，可以创建自定义 INF 文件。

创建文件 `rp1_usb.inf`：

```inf
[Version]
Signature="$Windows NT$"
Class=Ports
ClassGuid={4D36E978-E325-11CE-BFC1-08002BE10318}
Provider=%ManufacturerName%
CatalogFile=rp1_usb.cat
DriverVer=01/08/2025,1.0.0.0

[Manufacturer]
%ManufacturerName%=Standard,NTamd64

[Standard.NTamd64]
%DeviceName%=DriverInstall,USB\VID_2E8A&PID_000A&MI_00

[DriverInstall]
Include=mdmcpq.inf
CopyFiles=FakeModemCopyFileSection
AddReg=DriverInstall.AddReg

[DriverInstall.AddReg]
HKR,,DevLoader,,*ntkern
HKR,,NTMPDriver,,usbser.sys
HKR,,EnumPropPages32,,"MsPorts.dll,SerialPortPropPageProvider"

[DriverInstall.Services]
Include=mdmcpq.inf
AddService=usbser, 0x00000002, DriverService

[DriverService]
DisplayName=%ServiceName%
ServiceType=1
StartType=3
ErrorControl=1
ServiceBinary=%12%\usbser.sys

[Strings]
ManufacturerName="RP1 Embassy"
DeviceName="RP1 USB Serial Port"
ServiceName="USB Serial Driver"
```

使用方法：
1. 将上述内容保存为 `rp1_usb.inf`
2. 在设备管理器中右键未知设备
3. 更新驱动程序 → 浏览计算机
4. 选择 INF 文件所在目录
5. Windows 会自动找到并安装

## 🧪 测试步骤

### 测试 1：验证 COM 口

1. **找到 COM 口**
   - 设备管理器 → 端口 (COM 和 LPT)
   - 记下 COM 口号（如 COM8）

2. **连接串口**
   - 打开 PuTTY / TeraTerm
   - 选择 Serial
   - COM 口：COM8
   - 速率：115200

3. **测试回显**
   - 输入任何字符
   - 应该会回显（代码中实现了回显功能）

### 测试 2：验证键盘

1. **打开记事本**
   - 启动 Windows 记事本

2. **观察输出**
   - 每 5 秒应该自动输入一个 'H' 字符
   - 从日志可以看到：`[USB-Keyboard] [DEBUG] Sent 'H' key (count: X)`

### 测试 3：验证鼠标

1. **观察鼠标指针**
   - 每 3 秒鼠标应该向右移动 50 像素
   - 从日志可以看到：`[USB-Mouse] [DEBUG] Moved mouse (count: X)`

2. **如果看不到移动**
   - 确保鼠标在屏幕中央
   - 不要手动移动鼠标
   - 观察 3 秒

## 📝 常见问题

### Q1: Zadig 安装后还是不识别

**A**: 尝试：
1. 重启电脑
2. 更换 USB 端口
3. 确保以管理员权限运行
4. 尝试方案 2（手动安装）

### Q2: 安装后 COM 口打不开

**A**: 可能被其他程序占用：
1. 关闭所有串口程序
2. 重新拔插 USB
3. 尝试不同的 COM 口

### Q3: HID 键盘/鼠标不工作

**A**: 
1. 检查设备管理器中 HID 设备是否正常
2. 可能需要禁用并重新启用
3. 查看 UART 日志确认是否在发送数据

### Q4: 为什么 U 盘没有出现？

**A**: U 盘功能目前**不可用**：
- embassy-usb 0.3.0 不支持 MSC (大容量存储)
- 代码中只创建了占位符
- 需要等待 embassy-usb 未来版本支持

## 🎯 预期最终状态

### 设备管理器

```
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8) ✅

人体学输入设备
  ├─ HID-compliant keyboard ✅
  └─ HID-compliant mouse ✅

通用串行总线设备
  └─ USB Composite Device ✅
```

### UART 日志（你已经看到了）

```
[      64ms] [USB] [INFO ] All USB tasks starting...
[      64ms] [USB-Serial] [INFO ] CDC-ACM task running
[     286ms] [USB-Serial] [INFO ] Host connected! Echo mode active.
[    5115ms] [USB-Keyboard] [DEBUG] Sent 'H' key (count: 1)
[    3065ms] [USB-Mouse] [DEBUG] Moved mouse (count: 1)
```

### 功能验证

- ✅ COM8 可以连接并回显
- ✅ 每 5 秒自动输入 'H'
- ✅ 每 3 秒鼠标向右移动 50px
- ❌ U 盘不可用（正常，功能未实现）

## 🔄 重新编译和烧录

如果需要应用新的改进（鼠标移动距离增大）：

```bash
# 1. 重新编译
cargo build --release

# 2. 烧录
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy

# 或使用 Makefile
make flash
```

## 💡 优化建议

### 1. 调整键盘发送间隔

编辑 `src/usb/hid.rs`：

```rust
// 改变发送频率
Timer::after_secs(10).await;  // 从 5 秒改为 10 秒
```

### 2. 调整鼠标移动距离

编辑 `src/usb/hid.rs`：

```rust
// 改变移动距离
let report = [0, 100, 0, 0]; // 移动 100 像素（更明显）
```

### 3. 停止自动发送

如果不想要自动发送功能，可以注释掉相关代码，或者在循环中增加一个标志位。

## 🎉 总结

你的硬件和固件都**工作正常**！

问题只是 Windows 驱动识别：
1. ✅ 使用 Zadig 安装 usbser 驱动
2. ✅ 或者手动安装 USB Serial Device 驱动
3. ✅ 断开重连 USB
4. ✅ 在设备管理器中确认

之后你就可以：
- 通过 COM8 连接并测试回显
- 看到键盘自动输入
- 看到鼠标自动移动（现在是 50 像素，更明显了）

**注意**：U 盘功能目前不可用（embassy-usb 限制），但 CDC-ACM 串口和 HID 都可以正常工作！

