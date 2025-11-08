# RP1 Embassy - Raspberry Pi Pico USB HID + CDC-ACM

基于 Embassy 框架的 Raspberry Pi Pico 项目，支持 USB HID 键盘和可选的 CDC-ACM 串口功能。

## 功能特性

- ✅ USB HID 键盘 (始终启用)
- 🔧 USB CDC-ACM 串口 (可选，用于调试)
- 📊 UART0 日志输出
- ⚡ 双核支持 (Core0 + Core1)

## 编译

### 仅 HID 键盘模式 (默认)

大多数情况下推荐使用此模式，体积更小且更稳定：

```bash
cargo build --release
```

### HID + CDC 串口模式 (用于调试)

如果需要 USB 串口功能，启用 `usb-serial` feature：

```bash
cargo build --release --features usb-serial
```

## USB 配置说明

### 默认模式 (仅 HID)
- **设备类型**: RP1 HID Keyboard
- **USB ID**: VID:0x2E8A PID:0x000A
- **USB Class**: 0x00 (由接口定义)
- **接口**: 单个 HID 接口
- **兼容性**: ✓ Windows / ✓ Linux / ✓ macOS

### 串口模式 (HID + CDC)
- **设备类型**: RP1 Serial + Keyboard
- **USB ID**: VID:0x2E8A PID:**0x000C** ← 不同 PID 避免驱动冲突
- **USB Class**: 0xEF (Miscellaneous) / 0x02 (Common) / 0x01 (IAD)
- **接口**: CDC-ACM + HID (使用接口关联描述符 IAD)
- **兼容性**: ✓ Linux / ✓ macOS / ⚠️ Windows (需要清除驱动缓存)

## 烧录

```bash
# 使用 probe-rs
cargo run --release

# 或使用 elf2uf2-rs 
cargo install elf2uf2-rs
elf2uf2-rs target/thumbv6m-none-eabi/release/rp1-embassy
```

## 硬件连接

- **UART0 (日志输出)**
  - TX: GPIO0
  - RX: GPIO1
  - 波特率: 115200

- **USB**
  - Pico 板载 USB 端口

## Windows 用户注意事项

### 🔥 最新修复（v3 - 2025-11-08）

已修改 USB Device Class 配置以提高 Windows 兼容性：
- **之前**: 0xEF (Miscellaneous) - 某些 Windows 版本不兼容
- **现在**: 0x02 (CDC) - 更好的兼容性

### 如果遇到"错误 10"

1. **清除 USB 驱动缓存**（关键！）
   - 设备管理器 → 查看 → 显示隐藏的设备
   - 卸载所有 VID:2E8A 设备
   - ✅ 勾选"删除此设备的驱动程序软件"
   - **重启 Windows**
   
2. **烧录最新固件**
   ```bash
   make clean
   make build-serial  # 使用新的 Device Class 0x02
   ```

3. **快速测试指南**
   - `QUICK_TEST.md` - 3 步骤快速验证
   - `WINDOWS_CDC_FIX_V2.md` - 完整修复方案

**建议**：日常使用默认 HID 模式，通过 UART0 查看日志。

## 开发说明

### 项目结构

```
src/
├── main.rs           # 主程序入口
├── banner.rs         # 启动横幅
├── config.rs         # 配置常量
├── logger.rs         # UART 日志系统
├── tasks/            # 异步任务
└── usb/              # USB 功能
    ├── mod.rs        # USB 配置
    ├── hid.rs        # HID 键盘
    └── serial.rs     # CDC-ACM 串口 (可选)
```

### Feature Flags

- `usb-serial`: 启用 USB CDC-ACM 串口功能

### 日志等级

可以在 `config.rs` 中配置 UART 日志等级：
- `DEBUG`: 详细调试信息
- `INFO`: 一般信息 (默认)
- `WARN`: 警告
- `ERROR`: 错误

## 许可证

MIT License

