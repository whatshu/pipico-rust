# RP1-Embassy 文档中心

欢迎查看 RP1-Embassy 项目的完整文档。

## 📚 文档索引

### 🚀 入门指南

| 文档 | 说明 |
|------|------|
| [QUICK_START.md](QUICK_START.md) | **快速开始指南** - 从零开始的详细教程 |
| [ARCHITECTURE.md](ARCHITECTURE.md) | **架构说明** - 项目架构和设计理念 |

### 🔌 USB 功能

| 文档 | 说明 |
|------|------|
| [USB_README.md](USB_README.md) | **USB 功能说明** - CDC-ACM、HID 键盘、HID 鼠标 |

### 📝 日志系统

| 文档 | 说明 |
|------|------|
| [LOG_ASYNC_README.md](LOG_ASYNC_README.md) | **异步日志系统** - 日志架构和使用方法 |

### 🪟 Windows 支持

| 文档 | 说明 |
|------|------|
| [WINDOWS_DRIVER_FIX.md](WINDOWS_DRIVER_FIX.md) | **驱动安装指南** - Windows USB 驱动安装 |
| [WINDOWS_QUICK_FIX.md](WINDOWS_QUICK_FIX.md) | **快速修复指南** - Windows 常见问题快速解决 |

### 🔧 故障排除

| 文档 | 说明 |
|------|------|
| [TROUBLESHOOTING.md](TROUBLESHOOTING.md) | **故障排除** - 常见问题和解决方案 |

### 📊 改进记录

| 文档 | 说明 |
|------|------|
| [IMPROVEMENTS_SUMMARY.md](IMPROVEMENTS_SUMMARY.md) | **改进总结** - 项目改进和优化记录 |

## 🎯 推荐阅读顺序

### 新用户（第一次使用）

1. 📖 **[QUICK_START.md](QUICK_START.md)** - 从这里开始
2. 🔌 **[USB_README.md](USB_README.md)** - 了解 USB 功能
3. 🪟 **[WINDOWS_DRIVER_FIX.md](WINDOWS_DRIVER_FIX.md)** - Windows 用户必读
4. 🔧 **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** - 遇到问题时查看

### 开发者（深入了解）

1. 🏗️ **[ARCHITECTURE.md](ARCHITECTURE.md)** - 理解项目架构
2. 📝 **[LOG_ASYNC_README.md](LOG_ASYNC_README.md)** - 了解日志系统
3. 📊 **[IMPROVEMENTS_SUMMARY.md](IMPROVEMENTS_SUMMARY.md)** - 查看改进历史

### Windows 用户（驱动问题）

1. 🚀 **[WINDOWS_QUICK_FIX.md](WINDOWS_QUICK_FIX.md)** - 快速解决常见问题
2. 🔧 **[WINDOWS_DRIVER_FIX.md](WINDOWS_DRIVER_FIX.md)** - 详细的驱动安装步骤

## 📝 文档概览

### QUICK_START.md
**快速开始指南**
- 完整的入门教程
- 构建和烧录步骤
- 测试和验证方法
- 适合第一次使用的用户

### USB_README.md
**USB 功能说明**
- USB CDC-ACM 虚拟串口
- USB HID 键盘功能
- USB HID 鼠标功能
- USB 配置详解
- 代码示例

### ARCHITECTURE.md
**架构说明**
- 项目整体架构
- 模块划分
- 异步任务设计
- 双核协作机制
- 设计理念

### LOG_ASYNC_README.md
**异步日志系统**
- 日志系统架构
- 同步和异步日志宏
- Channel 通信机制
- 使用示例
- 性能考虑

### WINDOWS_DRIVER_FIX.md
**Windows 驱动安装指南**
- 驱动问题诊断
- Zadig 安装教程
- 手动驱动安装
- 设备管理器配置
- 常见错误代码解决

### WINDOWS_QUICK_FIX.md
**Windows 快速修复指南**
- 快速诊断清单
- 常见问题快速解决
- 最佳实践
- 故障排除流程

### TROUBLESHOOTING.md
**故障排除**
- 编译错误
- 烧录问题
- USB 识别问题
- 串口连接问题
- 性能问题

### IMPROVEMENTS_SUMMARY.md
**改进总结**
- 异步日志系统改进
- USB 功能实现
- 代码优化记录
- 文档改进
- 未来计划

## 🔍 快速查找

### 遇到问题？

| 问题类型 | 查看文档 |
|---------|---------|
| 编译错误 | [TROUBLESHOOTING.md](TROUBLESHOOTING.md) |
| USB 设备不识别 | [WINDOWS_DRIVER_FIX.md](WINDOWS_DRIVER_FIX.md) |
| COM 口无法连接 | [WINDOWS_QUICK_FIX.md](WINDOWS_QUICK_FIX.md) |
| 没有日志输出 | [TROUBLESHOOTING.md](TROUBLESHOOTING.md) |
| 键盘/鼠标不工作 | [USB_README.md](USB_README.md) |

### 想了解功能？

| 功能 | 查看文档 |
|------|---------|
| USB 串口 | [USB_README.md](USB_README.md) → CDC-ACM |
| USB 键盘 | [USB_README.md](USB_README.md) → HID 键盘 |
| USB 鼠标 | [USB_README.md](USB_README.md) → HID 鼠标 |
| 日志系统 | [LOG_ASYNC_README.md](LOG_ASYNC_README.md) |
| 双核运行 | [ARCHITECTURE.md](ARCHITECTURE.md) |

### 想修改代码？

| 修改内容 | 查看文档 |
|---------|---------|
| USB 功能 | [USB_README.md](USB_README.md) |
| 日志格式 | [LOG_ASYNC_README.md](LOG_ASYNC_README.md) |
| 任务行为 | [ARCHITECTURE.md](ARCHITECTURE.md) |
| 整体架构 | [ARCHITECTURE.md](ARCHITECTURE.md) |

## 📞 获取帮助

1. **查看相关文档** - 根据上面的索引找到对应文档
2. **检查故障排除** - [TROUBLESHOOTING.md](TROUBLESHOOTING.md) 包含常见问题
3. **查看 GitHub Issues** - 可能已有类似问题的解决方案
4. **提交新 Issue** - 如果问题未解决

## 🔄 文档更新

所有文档会随项目更新而更新。如果发现文档过时或有错误，欢迎：
- 提交 Issue 报告
- 提交 Pull Request 修正

## 📝 文档编写规范

本项目文档遵循以下规范：
- 使用 Markdown 格式
- 包含清晰的目录
- 提供代码示例
- 使用表格和列表提高可读性
- 包含故障排除章节

## 🎓 学习资源

### Embassy 框架
- [Embassy 官方文档](https://embassy.dev/)
- [Embassy GitHub](https://github.com/embassy-rs/embassy)

### RP2040
- [RP2040 数据手册](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
- [Raspberry Pi Pico 文档](https://www.raspberrypi.com/documentation/microcontrollers/)

### USB
- [USB 规范](https://www.usb.org/documents)
- [USB CDC-ACM 规范](https://www.usb.org/document-library/class-definitions-communication-devices-12)
- [USB HID 规范](https://www.usb.org/document-library/device-class-definition-hid-111)

---

**返回项目主页**：[../README.md](../README.md)

