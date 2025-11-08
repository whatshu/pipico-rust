# RP1-Embassy: RP2040 USB 复合设备

基于 Embassy 异步框架的 RP2040 USB 复合设备项目，支持双核运行、异步日志系统。

## ✨ 主要功能

### USB 功能
- ✅ **USB CDC-ACM** - 虚拟串口（回显模式）
- ✅ **USB HID 键盘** - 每 5 秒自动发送 'H' 键

### 系统功能
- ✅ **双核支持** - Core0 和 Core1 独立运行
- ✅ **异步日志** - 基于 Channel 的非阻塞日志系统
- ✅ **UART0 输出** - 115200 波特率调试日志（GPIO0/GPIO1）

## 🚀 快速开始

### 1. 构建项目

```bash
cargo build --release
```

### 2. 烧录固件

**方法 A：使用 Makefile**
```bash
make flash
```

**方法 B：手动烧录**
```bash
# 转换为 UF2 格式
elf2uf2-rs target/thumbv6m-none-eabi/release/rp1-embassy rp1-embassy.uf2

# 按住 BOOTSEL 按钮，连接 Pico
# 将 rp1-embassy.uf2 复制到 RPI-RP2 驱动器
```

**方法 C：使用 probe-rs（推荐）**
```bash
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/rp1-embassy
```

### 3. 查看日志

**通过 UART0（推荐用于调试）**：
```bash
# 连接 USB-UART 转换器到 GPIO0(TX) 和 GPIO1(RX)
# Linux/macOS
screen /dev/ttyUSB0 115200

# Windows
# 使用 PuTTY 连接对应 COM 口
```

**通过 USB CDC-ACM**：
- Windows: 使用 Zadig 安装 `usbser` 驱动后，连接 COM 口
- Linux: 连接 `/dev/ttyACM0`
- macOS: 连接 `/dev/cu.usbmodem*`

## 📦 硬件要求

- Raspberry Pi Pico 或其他 RP2040 开发板
- USB-UART 转换器（可选，用于查看 UART 日志）
- USB 数据线（用于 USB 功能和供电）

## 📋 硬件连接

### UART0 日志输出（可选）
```
USB-UART 转换器    RP2040
    TX      →     GPIO1 (RX)
    RX      ←     GPIO0 (TX)
    GND     →     GND
```

### USB 功能
- 使用 RP2040 的内置 USB（USB_DP/USB_DM）
- 直接通过 USB 连接到电脑

## 🔧 项目结构

```
rp1-embassy/
├── src/
│   ├── main.rs              # 主程序
│   ├── logger.rs            # 异步日志系统
│   ├── banner.rs            # 启动横幅
│   ├── config.rs            # 配置常量
│   ├── usb/
│   │   ├── mod.rs           # USB 模块
│   │   ├── serial.rs        # CDC-ACM 实现
│   │   └── hid.rs           # HID 实现
│   └── tasks/
│       ├── mod.rs           # 任务模块
│       ├── core0.rs         # Core 0 任务
│       └── core1.rs         # Core 1 任务
├── docs/                    # 文档目录（详细文档）
├── Cargo.toml               # 依赖配置
├── Makefile                 # 构建脚本
└── README.md                # 本文件
```

## 📚 文档

详细文档位于 `docs/` 目录：

- **[快速开始指南](docs/QUICK_START.md)** - 详细的入门教程
- **[USB 功能说明](docs/USB_README.md)** - USB 设备功能详解
- **[架构说明](docs/ARCHITECTURE.md)** - 项目架构设计
- **[日志系统](docs/LOG_ASYNC_README.md)** - 异步日志系统说明
- **[Windows 驱动修复](docs/WINDOWS_DRIVER_FIX.md)** - Windows 驱动安装指南
- **[故障排除](docs/TROUBLESHOOTING.md)** - 常见问题解决

完整文档索引：[docs/README.md](docs/README.md)

## 🛠️ 开发工具

### 必需工具
```bash
# Rust 工具链（thumbv6m-none-eabi）
rustup target add thumbv6m-none-eabi

# elf2uf2-rs（用于生成 UF2 文件）
cargo install elf2uf2-rs

# probe-rs（用于烧录和调试）
cargo install probe-rs --features cli
```

### Makefile 命令
```bash
make build          # 编译项目
make release        # 编译 release 版本
make flash          # 烧录到设备
make clean          # 清理构建文件
make size           # 查看二进制大小
```

## 📝 预期输出

### UART0 日志输出
```
=====================================
  RP2040 Dual Core UART Demo
  Embassy Async Framework
=====================================
[      12ms] [Main] [INFO ] System initialization starting...
[      12ms] [Main] [INFO ] Initializing USB composite device...
[      63ms] [USB] [INFO ] USB driver created
[      63ms] [USB] [INFO ] CDC-ACM serial port created
[      64ms] [USB] [INFO ] HID keyboard created
[     286ms] [USB-Serial] [INFO ] Host connected! Echo mode active.
[    5115ms] [USB-Keyboard] [DEBUG] Sent 'H' key (count: 1)
```

### USB 设备（Windows 设备管理器）
```
端口 (COM 和 LPT)
  └─ USB Serial Device (COM8)

人体学输入设备
  └─ HID-compliant keyboard
```

## 🧪 功能测试

### 测试 CDC-ACM 串口
1. 连接 COM 口（如 COM8）
2. 发送任何数据
3. 应该收到相同的回显数据

### 测试 HID 键盘
1. 打开记事本或文本编辑器
2. 每 5 秒会自动输入字母 'H'

## 🔍 核心依赖

- `embassy-executor` - 异步执行器
- `embassy-rp` - RP2040 HAL
- `embassy-time` - 时间和定时器
- `embassy-sync` - 同步原语（Channel、Mutex）
- `embassy-usb` - USB 栈
- `usbd-hid` - HID 描述符
- `defmt` - 高效日志框架

## 📄 许可证

MIT 或 Apache-2.0（与 Embassy 框架一致）

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 获取帮助

- 查看 [故障排除文档](docs/TROUBLESHOOTING.md)
- 查看 [完整文档](docs/README.md)
- 提交 GitHub Issue

---

**注意**：本项目基于 Embassy 异步框架，需要对 Rust async/await 有基本了解。
