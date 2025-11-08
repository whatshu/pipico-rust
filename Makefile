.PHONY: all build clean

TARGET = rp1-embassy
BUILD_MODE = release
ELF = target/thumbv6m-none-eabi/$(BUILD_MODE)/$(TARGET)
UF2 = $(TARGET).uf2

all: build

build:
	@echo "Building $(TARGET)..."
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
	@echo "  Flash size: $$(ls -lh $(ELF) | awk '{print $$5}')"

clean:
	cargo clean
	rm -f $(UF2)

