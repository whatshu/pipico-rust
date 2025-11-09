# RP1 Embassy - Raspberry Pi Pico USB HID Keyboard

基于 Embassy 异步框架的 Raspberry Pi Pico USB HID 键盘项目。

## 功能特性

- ✅ USB HID 键盘功能
- 📊 UART0 日志输出 (115200 波特率)
- ⚡ 双核支持 (Core0 + Core1)

## 硬件要求

- Raspberry Pi Pico 开发板
- USB 连接线（用于烧录和 HID 键盘功能）
- （可选）UART 转 USB 适配器（用于查看日志）

## 硬件连接

### UART0 日志输出
- TX: GPIO0
- RX: GPIO1
- 波特率: 115200

### USB
- 使用 Pico 板载 USB 端口

## 编译和烧录

### 编译
```bash
cargo build --release
```

### 烧录方式 1: 使用 probe-rs（推荐用于调试）
```bash
cargo run --release
```

### 烧录方式 2: 使用 UF2 模式
```bash
# 安装 elf2uf2-rs
cargo install elf2uf2-rs

# 转换并烧录
elf2uf2-rs target/thumbv6m-none-eabi/release/rp1-embassy
```

或使用 Makefile：
```bash
make build    # 编译
make flash    # 转换为 UF2 格式（将生成 rp1-embassy.uf2）
```

然后：
1. 按住 Pico 上的 BOOTSEL 按钮
2. 插入 USB 连接线
3. 释放按钮，Pico 会作为 USB 存储设备出现
4. 将 `rp1-embassy.uf2` 文件复制到 Pico 磁盘
5. 设备会自动重启并开始运行

## USB 设备信息

- **VID**: 0x2E8A (Raspberry Pi)
- **PID**: 0x000A (HID Keyboard)
- **产品名称**: RP1 HID Keyboard
- **制造商**: RP1 Embassy

## 功能说明

### USB HID 键盘
- 设备启动后会自动枚举为 USB HID 键盘
- 每 5 秒自动发送一次 'H' 键（演示用）
- 可以修改 `src/usb/hid.rs` 中的 `run_keyboard()` 函数来自定义键盘行为

### 日志系统
- 使用 UART0 输出日志信息
- 可以通过串口工具（如 minicom、screen、PuTTY）查看日志
- 日志级别可在 `src/config.rs` 中配置

### 双核任务
- Core0: 运行主要的系统任务和 USB 功能
- Core1: 运行独立的任务（可在 `src/tasks/mod.rs` 中自定义）

## 项目结构

```
src/
├── main.rs           # 主程序入口，初始化硬件和任务
├── banner.rs         # 启动横幅
├── config.rs         # 配置常量（UART 波特率、日志级别等）
├── logger.rs         # UART 异步日志系统
├── tasks/            # 异步任务模块
│   └── mod.rs        # Core0 和 Core1 任务
└── usb/              # USB 功能模块
    ├── mod.rs        # USB 配置和初始化
    └── hid.rs        # HID 键盘实现
```

## 依赖项

主要依赖：
- `embassy-executor`: 异步执行器
- `embassy-time`: 时间和定时器
- `embassy-rp`: Raspberry Pi Pico HAL
- `embassy-usb`: USB 设备栈
- `usbd-hid`: HID 类描述符
- `defmt`: 轻量级日志框架

完整依赖列表请查看 `Cargo.toml`。

## 自定义开发

### 修改键盘行为

编辑 `src/usb/hid.rs` 中的 `run_keyboard()` 函数：

```rust
pub async fn run_keyboard<'d, D: Driver<'d>>(
    mut keyboard: HidWriter<'d, D, 8>,
) {
    loop {
        // 在这里实现你的键盘逻辑
        // 例如：读取 GPIO 输入并发送对应的按键
        
        // HID 键盘报告格式：[modifier, reserved, key1, key2, ...]
        let report = [0, 0, 0x04, 0, 0, 0, 0, 0]; // 发送 'A' 键
        keyboard.write(&report).await;
        
        Timer::after_millis(50).await;
        
        // 释放按键
        let release = [0, 0, 0, 0, 0, 0, 0, 0];
        keyboard.write(&release).await;
        
        Timer::after_secs(1).await;
    }
}
```

### HID 键码参考

常用键码（第3个字节）：
- `0x04`: A
- `0x05`: B
- ...
- `0x0B`: H
- `0x1C`: Y
- `0x1D`: Z
- `0x27`: 0
- `0x28`: Enter
- `0x2C`: Space

Modifier 键（第1个字节）：
- `0x01`: Left Ctrl
- `0x02`: Left Shift
- `0x04`: Left Alt
- `0x08`: Left GUI (Windows/Command)

完整的 HID 键码表可参考 [USB HID Usage Tables](https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf)。

## 调试

### 查看日志输出

使用 USB-UART 适配器连接到 GPIO0 (TX) 和 GPIO1 (RX)：

```bash
# Linux/macOS
screen /dev/ttyUSB0 115200

# 或使用 minicom
minicom -D /dev/ttyUSB0 -b 115200

# Windows（使用 PuTTY 或 TeraTerm）
# 选择对应的 COM 口，波特率 115200
```

### 使用 defmt 日志

项目使用 `defmt-rtt` 进行调试输出。使用 `probe-rs` 时可以自动看到 defmt 日志：

```bash
cargo run --release
```

## 常见问题

### Q: 键盘无法识别？
A: 
1. 确认设备已正确枚举（在设备管理器或 `lsusb` 中可见）
2. 检查 USB 连接线是否正常
3. 尝试重新插拔 USB 连接线

### Q: 如何修改演示行为？
A: 编辑 `src/usb/hid.rs` 中的 `run_keyboard()` 函数，修改按键发送逻辑和间隔时间。

### Q: 如何添加更多功能？
A: 
- 添加 GPIO 输入: 在 `src/tasks/mod.rs` 中添加 GPIO 读取任务
- 添加新的 USB 功能: 在 `src/usb/` 下创建新的模块
- 修改日志级别: 编辑 `src/config.rs`

## 许可证

MIT License

## 相关资源

- [Embassy 文档](https://embassy.dev/)
- [Raspberry Pi Pico 数据手册](https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf)
- [USB HID Usage Tables](https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf)
