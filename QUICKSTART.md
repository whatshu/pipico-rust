# 快速开始指南

## 快速编译

### 默认模式（推荐）- 仅 HID 键盘

```bash
make build
# 或
cargo build --release
```

生成文件：`rp1-embassy.uf2`（约 908KB）

### 调试模式 - HID + CDC 串口

```bash
make build-serial
# 或
cargo build --release --features usb-serial
```

## 快速烧录

1. 按住 Pico 上的 **BOOTSEL** 按钮
2. 插入 USB 线
3. 复制 `rp1-embassy.uf2` 到出现的 RPI-RP2 磁盘
4. 完成！设备会自动重启

## 验证设备

### Linux
```bash
# 查看 USB 设备
lsusb | grep 2e8a

# 查看 HID 设备
ls /dev/hidraw*

# 查看串口（如果启用了 usb-serial）
ls /dev/ttyACM*
```

### Windows
打开"设备管理器"查看：
- **人体学输入设备** → USB HID 键盘
- **端口 (COM 和 LPT)**（如果启用了 usb-serial）→ USB Serial Device (COMx)

### macOS
```bash
# 查看 USB 设备
system_profiler SPUSBDataType | grep -A 10 "HID Keyboard"

# 查看串口（如果启用了 usb-serial）
ls /dev/cu.usbmodem*
```

## 查看日志

通过 UART0（GPIO0=TX, GPIO1=RX）连接 USB-TTL 转换器：

```bash
# Linux/macOS
screen /dev/ttyUSB0 115200

# Windows
# 使用 PuTTY 或其他串口工具连接到对应的 COM 端口
```

## 测试 USB 串口（usb-serial 模式）

```bash
# Linux/macOS
echo "Hello" > /dev/ttyACM0
cat /dev/ttyACM0

# 输入的内容会被回显
```

## 常见问题

**Q: 编译失败？**
```bash
# 清理并重新编译
make clean
make build
```

**Q: Windows 显示错误 10 "该设备无法启动"？**

这是最常见的 Windows CDC-ACM 问题，**已修复**！

**解决步骤：**
1. **清除 Windows USB 驱动缓存**（最重要！）
   - 打开设备管理器
   - 找到问题设备并右键"卸载设备"
   - **勾选**"删除此设备的驱动程序软件"
   - 拔出 Pico，等待 5 秒，重新插入

2. **使用最新固件**
   ```bash
   make clean
   make build-serial  # 使用新的 USB PID:0x000C
   ```

3. **查看详细指南**
   - 参考 `WINDOWS_TROUBLESHOOTING.md` 获取完整步骤

**为什么会发生？**
- Windows 缓存了旧的 USB 设备信息（旧 PID）
- 新版本使用不同的 PID (0x000C) 来避免冲突

**Q: 如何切换模式？**
```bash
# 切换到 HID 模式（PID: 0x000A）
make build

# 切换到 HID + 串口模式（PID: 0x000C）
make build-serial

# 然后重新烧录固件
```

## 更多信息

- 详细说明：`README.md`
- 使用指南：`USAGE.md`
- 更改日志：`CHANGELOG.md`

