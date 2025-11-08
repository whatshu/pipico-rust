# 快速测试新固件 (Device Class 0x02)

## 🔥 重要变更

刚刚修改了 USB Device Class 配置：

**之前：** 0xEF (Miscellaneous Device)  
**现在：** 0x02 (CDC - Communications Device Class)

这应该能解决 Windows 错误 10 问题！

## 快速测试（3 步骤）

### 1️⃣ 彻底清除旧驱动 ⚠️

```
打开设备管理器
查看 → 显示隐藏的设备
找到所有 VID:2E8A 的设备（包括隐藏的）
逐个右键 → 卸载设备
✅ 勾选 "删除此设备的驱动程序软件"
拔出 Pico，等待 30 秒
❗ 重启 Windows（强烈建议）
```

### 2️⃣ 烧录新固件

```bash
# 当前目录的 rp1-embassy.uf2 已是最新版本
# 或重新编译
make clean
make build-serial
```

烧录：
1. 按住 BOOTSEL，插入 Pico
2. 复制 rp1-embassy.uf2 到 RPI-RP2
3. 等待重启

### 3️⃣ 验证

设备管理器应该显示：

✅ **成功：**
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COMx)  [无感叹号！]
```

❌ **失败（如果仍有问题）：**

请查看 `WINDOWS_CDC_FIX_V2.md` 获取：
- 详细的诊断步骤
- 备选配置方案
- 诊断信息收集方法

## 技术变更

```diff
# src/usb/mod.rs

- config.device_class = 0xEF;    // Miscellaneous Device
- config.device_sub_class = 0x02; // Common Class
- config.device_protocol = 0x01;  // IAD

+ config.device_class = 0x02;    // CDC
+ config.device_sub_class = 0x00;
+ config.device_protocol = 0x00;
```

## 为什么这样改？

- 更直接地表明设备是 CDC（串口）
- 某些 Windows 版本对 0x02 的支持更好
- 0xEF 需要 Windows Vista SP2+，可能在某些系统上有问题

## 如果仍然失败

查看详细文档：
- `WINDOWS_CDC_FIX_V2.md` - 完整的修复方案和诊断
- `WINDOWS_TROUBLESHOOTING.md` - 通用故障排除

或者提供以下信息：
1. Windows 版本
2. USB 描述符（从 Linux/macOS 获取）
3. 设备管理器错误详情
4. Windows 事件日志

祝好运！🍀

