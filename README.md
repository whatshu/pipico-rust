# RP1-Embassy: RP2040 双核 Blink 示例

基于 Embassy 异步框架的 RP2040 双核 LED 闪烁示例项目。

## 功能

- 使用 Embassy 异步执行器
- 启用 RP2040 双核功能
- Core 0: 控制 PIN_25 (板载 LED)，250ms 闪烁间隔
- Core 1: 控制 PIN_0，500ms 闪烁间隔
- 使用 defmt 进行日志输出

## 硬件要求

- Raspberry Pi Pico 或其他 RP2040 开发板

## 快速开始

### 构建项目

```bash
make build
```

这将自动：
1. 编译项目 (release 模式)
2. 生成 `rp1-embassy.uf2` 文件

### 烧录到设备

1. 按住 Pico 的 BOOTSEL 按钮，连接到电脑
2. Pico 会显示为 USB 存储设备
3. 将 `rp1-embassy.uf2` 文件复制到该设备
4. Pico 自动重启并运行程序

## 核心依赖

- embassy-executor: 异步执行器
- embassy-rp: RP2040 HAL
- embassy-time: 时间和定时器
- defmt: 高效日志框架

## 项目结构

```
rp1-embassy/
├── src/main.rs           # 主程序
├── Cargo.toml            # 依赖配置
├── .cargo/config.toml    # 构建配置
├── memory.x              # 内存布局
├── build.rs              # 构建脚本
└── Makefile              # 构建工具
```

