.PHONY: all build build-debug build-serial clean help

TARGET = rp1-embassy
BUILD_MODE = release
ELF = target/thumbv6m-none-eabi/$(BUILD_MODE)/$(TARGET)
UF2 = $(TARGET).uf2

all: build

# 默认构建：仅 HID 键盘
build:
	@echo "Building $(TARGET) (HID only)..."
	cargo build --$(BUILD_MODE)
	@echo ""
	@echo "Converting to UF2 format..."
	@if ! command -v elf2uf2-rs >/dev/null 2>&1; then \
		echo "Error: elf2uf2-rs not found"; \
		echo "Install with: cargo install elf2uf2-rs --locked"; \
		exit 1; \
	fi
	elf2uf2-rs $(ELF) $(UF2)
	@echo ""
	@echo "✓ Build complete: $(UF2)"
	@echo "  Mode: HID Keyboard only"
	@echo "  Flash size: $$(ls -lh $(ELF) | awk '{print $$5}')"

# 带串口的构建：HID + CDC-ACM
build-serial:
	@echo "Building $(TARGET) (HID + CDC-ACM Serial)..."
	cargo build --$(BUILD_MODE) --features usb-serial
	@echo ""
	@echo "Converting to UF2 format..."
	@if ! command -v elf2uf2-rs >/dev/null 2>&1; then \
		echo "Error: elf2uf2-rs not found"; \
		echo "Install with: cargo install elf2uf2-rs --locked"; \
		exit 1; \
	fi
	elf2uf2-rs $(ELF) $(UF2)
	@echo ""
	@echo "✓ Build complete: $(UF2)"
	@echo "  Mode: HID Keyboard + CDC-ACM Serial"
	@echo "  Flash size: $$(ls -lh $(ELF) | awk '{print $$5}')"

# Debug 构建
build-debug:
	@echo "Building $(TARGET) (debug, HID only)..."
	$(eval BUILD_MODE = debug)
	$(eval ELF = target/thumbv6m-none-eabi/debug/$(TARGET))
	cargo build
	@echo ""
	@echo "Converting to UF2 format..."
	elf2uf2-rs $(ELF) $(UF2)
	@echo "✓ Debug build complete: $(UF2)"

clean:
	cargo clean
	rm -f $(UF2)

help:
	@echo "Available targets:"
	@echo "  make build         - 构建 HID 键盘模式 (默认，推荐)"
	@echo "  make build-serial  - 构建 HID + CDC-ACM 串口模式 (用于调试)"
	@echo "  make build-debug   - 构建 debug 版本"
	@echo "  make clean         - 清理构建文件"
	@echo ""
	@echo "烧录方法："
	@echo "  1. 按住 BOOTSEL 按钮，插入 Pico"
	@echo "  2. 将生成的 $(UF2) 文件复制到 RPI-RP2 磁盘"

