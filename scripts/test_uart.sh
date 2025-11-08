#!/bin/bash
# UART 测试脚本 - 检查串口输出

set -e

echo "================================"
echo "  RP2040 UART 测试脚本"
echo "================================"
echo ""

# 检测串口设备
echo "1. 检测可用的串口设备..."
echo ""

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    PORTS=$(ls /dev/ttyUSB* /dev/ttyACM* 2>/dev/null || true)
    if [ -z "$PORTS" ]; then
        echo "❌ 未检测到串口设备！"
        echo ""
        echo "可能的原因："
        echo "- 设备未连接"
        echo "- USB 线缆问题"
        echo "- 需要安装驱动"
        echo "- 权限不足（尝试：sudo usermod -a -G dialout $USER）"
        exit 1
    fi
    
    echo "✅ 检测到以下串口设备："
    for port in $PORTS; do
        echo "   - $port"
    done
    
    # 选择第一个设备
    PORT=$(echo $PORTS | awk '{print $1}')
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    PORTS=$(ls /dev/tty.usb* /dev/tty.SLAB* 2>/dev/null || true)
    if [ -z "$PORTS" ]; then
        echo "❌ 未检测到串口设备！"
        exit 1
    fi
    
    echo "✅ 检测到以下串口设备："
    for port in $PORTS; do
        echo "   - $port"
    done
    
    PORT=$(echo $PORTS | awk '{print $1}')
else
    echo "❌ 不支持的操作系统: $OSTYPE"
    echo "请在 Linux 或 macOS 上运行此脚本"
    echo ""
    echo "Windows 用户请使用："
    echo "- PuTTY"
    echo "- TeraTerm"
    echo "- Arduino Serial Monitor"
    exit 1
fi

echo ""
echo "2. 使用串口: $PORT"
echo "   波特率: 115200"
echo "   数据格式: 8N1"
echo ""

# 检查设备权限
if [ ! -r "$PORT" ]; then
    echo "❌ 没有读取权限！"
    echo ""
    echo "解决方法："
    echo "  sudo chmod 666 $PORT"
    echo "或"
    echo "  sudo usermod -a -G dialout $USER"
    echo "  (然后注销并重新登录)"
    exit 1
fi

echo "3. 开始监听串口输出..."
echo "   按 Ctrl+C 退出"
echo ""
echo "================================"
echo ""

# 使用 cat 读取串口（简单但有效）
# 如果安装了 minicom 或 screen，也可以使用它们

if command -v screen &> /dev/null; then
    echo "使用 screen (按 Ctrl+A 然后按 K 退出)"
    screen $PORT 115200
elif command -v minicom &> /dev/null; then
    echo "使用 minicom (按 Ctrl+A 然后按 X 退出)"
    minicom -D $PORT -b 115200
else
    echo "使用 cat (按 Ctrl+C 退出)"
    echo "提示: 安装 screen 或 minicom 可以获得更好的体验"
    echo "      sudo apt install screen  # Debian/Ubuntu"
    echo "      brew install screen       # macOS"
    echo ""
    
    # 配置串口参数
    stty -F $PORT 115200 cs8 -cstopb -parenb
    
    # 读取数据
    cat $PORT
fi

