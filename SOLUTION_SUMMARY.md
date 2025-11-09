# Windows CDC + HID 解决方案总结

## 🎯 问题

在 Windows 上实现 USB CDC-ACM (串口) + HID (键盘) 复合设备时遇到兼容性问题。

## 🔍 问题演进

### 尝试 1: Device Class 0xEF (Miscellaneous + IAD)
```
config.device_class = 0xEF;
config.device_sub_class = 0x02;
config.device_protocol = 0x01;
```
**结果：**
- ✅ HID 工作
- ❌ CDC 不工作（Windows 错误 10）
- 原因：某些 Windows 版本对 0xEF 的 CDC 支持不完善

### 尝试 2: Device Class 0x02 (CDC)
```
config.device_class = 0x02;
config.device_sub_class = 0x00;
config.device_protocol = 0x00;
```
**结果：**
- ✅ CDC 工作
- ❌ HID 不工作
- 原因：整个设备被 Windows 识别为纯 CDC 设备，HID 接口被忽略

### ✅ 最终方案: Device Class 0x00 (Interface Defined)
```
config.device_class = 0x00;
config.device_sub_class = 0x00;
config.device_protocol = 0x00;
```
**结果：**
- ✅ CDC 工作（COM 端口）
- ✅ HID 工作（键盘设备）
- 原因：让每个接口自己声明类型，Windows 分别识别

## 💡 技术原理

### Device Class 的作用

**Device Class 在设备描述符中的位置：**
```
Device Descriptor:
  bDeviceClass        ← 这里！
  bDeviceSubClass
  bDeviceProtocol
  ...
  
Configuration Descriptor:
  Interface 0 Descriptor:
    bInterfaceClass   ← 或这里！
  Interface 1 Descriptor:
    bInterfaceClass   ← 或这里！
```

**三种策略：**

| Device Class | 含义 | Windows 行为 |
|--------------|------|-------------|
| 0x00 | 由接口定义 | 查看每个接口的类型，分别处理 ✅ |
| 0x02 | 整个设备是 CDC | 把所有接口都当作 CDC，忽略 HID ❌ |
| 0xEF | 复合设备(IAD) | 依赖 IAD 支持，某些系统有问题 ⚠️ |

### embassy-usb 的 IAD 支持

即使 Device Class 设置为 0x00，embassy-usb 仍会为 CDC-ACM 自动生成 IAD：

```
Interface Association Descriptor:
  bFirstInterface:     0      ← CDC 通信接口
  bInterfaceCount:     2      ← CDC 占用 2 个接口
  bFunctionClass:      0x02   ← CDC 类
  bFunctionSubClass:   0x02   ← ACM 子类
  bFunctionProtocol:   0x00

Interface 0: CDC Communication
Interface 1: CDC Data
Interface 2: HID Keyboard (独立)
```

**关键点：**
- IAD 只关联接口 0 和 1（CDC）
- 接口 2（HID）独立存在
- Windows 看到 Device Class 0x00，会检查每个接口
- CDC 通过 IAD 被正确识别为串口
- HID 被独立识别为键盘

## 📊 完整配置

### USB 配置（src/usb/mod.rs）
```rust
config.device_class = 0x00;    // 由接口定义 ✅
config.device_sub_class = 0x00;
config.device_protocol = 0x00;
config.max_power = 100;        // 200mA
config.max_packet_size_0 = 64;
```

### VID/PID
```
VID: 0x2E8A (Raspberry Pi)
PID: 0x000C (CDC + HID 复合设备)
PID: 0x000A (仅 HID)
```

### 接口创建顺序（src/main.rs）
```rust
// 1. 先创建 CDC-ACM（占用接口 0 和 1）
let cdc_acm = usb::serial::create_cdc_acm(&mut builder, cdc_state);

// 2. 后创建 HID（占用接口 2）
let keyboard = usb::hid::create_keyboard_hid(&mut builder, keyboard_state, hid_handler);

// 3. 构建设备
let usb_device = builder.build();
```

**顺序很重要！** CDC 必须先创建，这样 embassy-usb 才能正确生成 IAD。

## 🎓 经验教训

### 1. Device Class 的选择

**错误思路：**
- "复合设备应该用 0xEF" ❌
- "CDC 设备应该用 0x02" ❌

**正确思路：**
- 让接口自己声明类型 (0x00) ✅
- Device Class 只是一个"提示"，不是强制要求
- 接口级的描述符才是决定性的

### 2. IAD 的作用

IAD（Interface Association Descriptor）是为了：
- 将多个接口组合成一个功能
- 对于 CDC-ACM，需要 2 个接口（通信 + 数据）
- Windows 通过 IAD 知道这两个接口是一起的

**但是：**
- IAD 不需要 Device Class 是 0xEF
- 即使 Device Class 是 0x00，IAD 也能工作
- embassy-usb 会自动生成正确的 IAD

### 3. Windows 的特殊性

**为什么 Linux/macOS 不会有这个问题？**
- Linux/macOS 对 USB 设备的处理更灵活
- 它们会检查所有接口，不管 Device Class 是什么
- Windows 更依赖 Device Class 来决定使用什么驱动

**教训：**
- 不要过度依赖"标准配置"
- 实际测试比理论重要
- Windows 兼容性需要特别关注

## 🔧 调试方法

### 1. 使用 lsusb 查看描述符（Linux）
```bash
lsusb -v -d 2e8a:000c

# 关注这些字段：
# - bDeviceClass
# - Interface Association Descriptor
# - bInterfaceClass (每个接口)
```

### 2. Windows USBView
- 下载 Windows SDK 或单独的 USBView
- 查看完整的 USB 描述符
- 确认 IAD 是否存在和正确

### 3. 设备管理器技巧
```
查看 → 显示隐藏的设备
└─ 可以看到已卸载但未删除的设备
└─ 右键 → 属性 → 详细信息
   └─ 硬件 ID: VID_2E8A&PID_000C&MI_00 (MI = Interface)
```

### 4. PowerShell 诊断
```powershell
# 查看所有 2E8A 设备
Get-PnpDevice | Where-Object {$_.InstanceId -like "*VID_2E8A*"} | Format-List

# 查看错误设备
Get-PnpDevice | Where-Object {$_.Status -eq "Error"} | Format-List
```

## 📚 参考资源

### USB 规范
- [USB CDC Class 1.2](https://www.usb.org/document-library/class-definitions-communication-devices-12)
- [USB HID 1.11](https://www.usb.org/hid)
- [IAD ECN](https://www.usb.org/document-library/interface-association-descriptor-engineering-change-notice)

### Windows 文档
- [USB Composite Device Support](https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/support-for-interface-collections)
- [USB Device Class Definitions](https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/)

### embassy-usb
- [Embassy USB Documentation](https://docs.embassy.dev/embassy-usb/)
- [embassy-usb GitHub](https://github.com/embassy-rs/embassy/tree/main/embassy-usb)

## ✅ 验证清单

测试固件前：
- [ ] Device Class 设置为 0x00
- [ ] VID:PID 为 2E8A:000C
- [ ] CDC-ACM 在 HID 之前创建
- [ ] 已卸载所有旧的 VID_2E8A 设备
- [ ] 已勾选"删除驱动程序软件"
- [ ] 已重启 Windows

测试成功标志：
- [ ] 设备管理器显示 COM 端口，无感叹号
- [ ] 设备管理器显示 HID 设备，无感叹号
- [ ] 串口可以打开并通信
- [ ] HID 设备被正确识别

## 🎉 总结

**最佳实践：**
```rust
// 对于 CDC + HID 复合设备
config.device_class = 0x00;    // 由接口定义
config.device_sub_class = 0x00;
config.device_protocol = 0x00;

// 接口创建顺序：
// 1. CDC-ACM (interface 0, 1)
// 2. HID (interface 2)
// embassy-usb 会自动生成正确的 IAD
```

**为什么有效：**
- Windows 查看每个接口的类型
- CDC 接口通过 IAD 被正确关联
- HID 接口独立存在
- 两者不冲突，分别加载驱动

**关键因素：**
- Device Class: 0x00 ✅
- embassy-usb 的 IAD 自动生成 ✅
- 正确的接口创建顺序 ✅
- 清除旧的 Windows 驱动缓存 ✅

---

**版本**: v4 Final  
**日期**: 2025-11-08  
**状态**: 已验证 CDC ✅ + HID ✅  
**适用**: Windows 10/11, Linux, macOS  

