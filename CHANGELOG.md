# 更新日志

## [未发布]

### 移除
- 删除 USB HID 鼠标功能（由于兼容性问题）

### 文档
- 将所有文档整理到 `docs/` 目录
- 创建文档索引 `docs/README.md`
- 删除示例文件和临时文档
- 精简项目结构

### 优化
- 简化 USB 设备配置（只保留 CDC-ACM + HID 键盘）
- 提高 Windows 兼容性

## [当前版本]

### 功能
- ✅ USB CDC-ACM 虚拟串口（回显模式）
- ✅ USB HID 键盘（每 5 秒发送 'H'）
- ✅ 双核异步支持（Core0 + Core1）
- ✅ 异步日志系统（UART0 输出）
- ✅ 非阻塞日志宏（同步和异步版本）

### 技术栈
- Embassy 异步框架
- embassy-usb 0.3.0
- RP2040 HAL
- defmt 日志框架

### 已知限制
- MSC (U 盘) 功能未实现（embassy-usb 0.3.0 不支持）
- USB HID 鼠标已删除（兼容性考虑）

