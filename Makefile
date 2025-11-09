.PHONY: all build build-debug flash clean help

TARGET = rp1-embassy
BUILD_MODE = release
ELF = target/thumbv6m-none-eabi/$(BUILD_MODE)/$(TARGET)
UF2 = $(TARGET).uf2

all: build

# 默认构建
build:
	@echo "Building $(TARGET) (USB HID Keyboard)..."
	cargo build --$(BUILD_MODE)
	@echo "✓ Build complete"
	@echo "  Binary: $(ELF)"
	@echo "  Size: $$(ls -lh $(ELF) | awk '{print $$5}')"

# 构建并生成 UF2 格式
flash: build
	@echo ""
	@echo "Converting to UF2 format..."
	@if ! command -v elf2uf2-rs >/dev/null 2>&1; then \
		echo "Error: elf2uf2-rs not found"; \
		echo "Install with: cargo install elf2uf2-rs --locked"; \
		exit 1; \
	fi
	elf2uf2-rs $(ELF) $(UF2)
	@echo ""
	@echo "✓ UF2 file created: $(UF2)"
	@echo ""
	@echo "To flash:"
	@echo "  1. Hold BOOTSEL button and connect Pico"
	@echo "  2. Copy $(UF2) to RPI-RP2 drive"

# Debug 构建
build-debug:
	@echo "Building $(TARGET) (debug mode)..."
	$(eval BUILD_MODE = debug)
	$(eval ELF = target/thumbv6m-none-eabi/debug/$(TARGET))
	cargo build
	@echo "✓ Debug build complete: $(ELF)"

clean:
	cargo clean
	rm -f $(UF2)

help:
	@echo "Available targets:"
	@echo "  make build        - Build release binary"
	@echo "  make flash        - Build and convert to UF2 format"
	@echo "  make build-debug  - Build debug binary"
	@echo "  make clean        - Clean build artifacts"
	@echo ""
	@echo "Flash instructions:"
	@echo "  1. Hold BOOTSEL button on Pico"
	@echo "  2. Connect USB cable"
	@echo "  3. Copy $(UF2) to RPI-RP2 drive"

