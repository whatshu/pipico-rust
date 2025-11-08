# 更改日志

## [未发布] - 2025-11-08 v2

### 重要修复 - Windows CDC-ACM 兼容性

#### ✅ 使用不同的 USB PID 避免驱动冲突
- **HID 模式**: PID:0x000A
- **CDC+HID 模式**: PID:0x000C（新）
- 这解决了 Windows 驱动缓存导致的"错误 10"问题

#### ✅ 优化的产品名称
- HID 模式：`"RP1 HID Keyboard"`
- CDC+HID 模式：`"RP1 Serial + Keyboard"`

#### ✅ 添加详细的 Windows 故障排除文档
- 新增 `WINDOWS_TROUBLESHOOTING.md`
- 包含完整的驱动清除和测试步骤
- 提供多种诊断工具和方法

### 新增功能

- 添加 Cargo feature `usb-serial` 用于编译时控制 CDC-ACM 串口功能
- 更新 Makefile，添加 `build-serial` 和 `help` 目标
- 创建详细的 README.md、USAGE.md 和 QUICKSTART.md 文档

### 更改

- USB 配置现在根据是否启用 `usb-serial` feature 自动调整：
  - 仅 HID 模式：Device Class 0x00（由接口定义），PID:0x000A
  - 复合设备模式：Device Class 0xEF/0x02/0x01（IAD 支持），PID:0x000C
- 产品名称更具描述性
- 日志输出根据启用的功能自动调整

### 修复

- ✅ **关键修复**：Windows CDC-ACM "错误 10" 问题
  - 使用独立的 PID 避免驱动缓存冲突
  - 确保 USB 2.0 兼容性（避免 USB 2.1 的额外要求）
  - 正确的 IAD 配置和接口顺序
- 清理未使用的导入和静态变量
- 修复编译警告

### 文档

- 添加完整的 README.md，包含编译和使用说明
- 添加 USAGE.md，详细说明 USB 功能和故障排除
- 更新 Makefile，添加中文帮助信息

### 技术细节

**条件编译范围：**
- `src/usb/serial.rs`：整个模块仅在 `usb-serial` feature 启用时编译
- `src/usb/mod.rs`：根据 feature 导出不同的模块和常量
- `src/main.rs`：根据 feature 创建不同的 USB 设备配置

**USB 描述符改进：**
- 正确配置 IAD（Interface Association Descriptor）
- 使用标准的 Miscellaneous Device Class（0xEF）
- 确保 Windows/Linux/macOS 兼容性
- 使用 USB 2.0 规范（避免 USB 2.1 的额外要求）

**编译优化：**
- 默认编译不包含 CDC-ACM 代码，减小固件体积
- 使用 `--features usb-serial` 可按需启用串口功能

**Windows 兼容性改进：**
- 独立的 USB PID (0x000C) 用于 CDC+HID 模式
- 避免与 HID 模式 (0x000A) 的驱动冲突
- 详细的故障排除文档和清除驱动缓存步骤
- 支持 Windows 10/11

**推荐使用方式：**
- **生产/日常使用**：默认 HID 模式 (`make build`)
- **调试开发**：启用串口模式 (`make build-serial`)
- **Windows 用户**：首次使用串口模式时，务必清除旧的驱动缓存

## [0.1.0] - 初始版本

### 功能

- USB HID 键盘支持
- USB CDC-ACM 串口支持（始终启用）
- UART0 日志输出
- 双核支持（Core0 + Core1）
- Embassy async 框架

