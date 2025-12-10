# Unfound OS Makefile
# Based on ArceOS framework with UCache, UNotify, UVFS modules

# ============================================================================
# Configuration
# ============================================================================

# Architecture and Platform
export ARCH ?= riscv64
export PLAT ?= riscv64-qemu-virt
export LOG ?= debug
export SMP ?= 1
export MODE ?= release

# Paths
AX_ROOT := $(PWD)/arceos
APP_DIR := $(PWD)/apps
TOOLCHAIN_DIR := /home/hen/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14

# Export for ArceOS
export NO_AXSTD := y
export AX_LIB := axfeat
export A := $(APP_DIR)/unotify_test

# Features
export FEATURES := fp-simd,paging,alloc,fs

# Output files
DIR := $(shell basename $(PWD))
OUT_ELF := $(DIR)_$(ARCH)-qemu-virt.elf
OUT_BIN := $(DIR)_$(ARCH)-qemu-virt.bin

# Target triple
ifeq ($(ARCH), x86_64)
  TARGET := x86_64-unknown-none
else ifeq ($(ARCH), riscv64)
  TARGET := riscv64gc-unknown-none-elf
else ifeq ($(ARCH), aarch64)
  TARGET := aarch64-unknown-none
else ifeq ($(ARCH), loongarch64)
  TARGET := loongarch64-unknown-none
else
  $(error ARCH must be one of x86_64, riscv64, aarch64, loongarch64)
endif

# QEMU options
QEMU_ARGS ?=
BLK ?= y
NET ?= n
GRAPHIC ?= n
BUS ?= pci

# Extra config
EXTRA_CONFIG ?= $(PWD)/configs/$(ARCH).toml

# ============================================================================
# Phony Targets
# ============================================================================

.PHONY: all help build run clean test doc install-toolchain check-toolchain

# ============================================================================
# Main Targets
# ============================================================================

all: build

help:
	@echo "Unfound OS Build System"
	@echo "======================="
	@echo ""
	@echo "Targets:"
	@echo "  all              - Build the kernel (default)"
	@echo "  build            - Build kernel and applications"
	@echo "  run              - Build and run in QEMU"
	@echo "  test             - Run UNotify test"
	@echo "  test-cache       - Run UCache test"
	@echo "  clean            - Clean build artifacts"
	@echo "  doc              - Generate documentation"
	@echo "  check-toolchain  - Check RISC-V toolchain installation"
	@echo "  install-toolchain- Install RISC-V toolchain symlinks"
	@echo ""
	@echo "Variables:"
	@echo "  ARCH=$(ARCH)     - Target architecture"
	@echo "  LOG=$(LOG)       - Log level (off/error/warn/info/debug/trace)"
	@echo "  SMP=$(SMP)       - Number of CPUs"
	@echo "  BLK=$(BLK)       - Enable block device"
	@echo "  NET=$(NET)       - Enable network"
	@echo ""
	@echo "Examples:"
	@echo "  make run                    - Run UNotify test"
	@echo "  make test LOG=trace         - Run with trace logging"
	@echo "  make clean                  - Clean all build artifacts"

# ============================================================================
# Toolchain Setup
# ============================================================================

check-toolchain:
	@echo "Checking RISC-V toolchain..."
	@if [ ! -d "$(TOOLCHAIN_DIR)" ]; then \
		echo "ERROR: RISC-V toolchain not found at $(TOOLCHAIN_DIR)"; \
		exit 1; \
	fi
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	if command -v riscv64-unknown-elf-gcc >/dev/null 2>&1; then \
		echo "✓ riscv64-unknown-elf-gcc found"; \
	else \
		echo "✗ riscv64-unknown-elf-gcc not found"; \
		exit 1; \
	fi

install-toolchain:
	@echo "Installing RISC-V toolchain symlinks..."
	@cd $(TOOLCHAIN_DIR)/bin && \
	ln -sf riscv64-unknown-elf-gcc riscv64-linux-musl-gcc && \
	ln -sf riscv64-unknown-elf-ar riscv64-linux-musl-ar && \
	ln -sf riscv64-unknown-elf-ld riscv64-linux-musl-ld && \
	ln -sf riscv64-unknown-elf-objcopy riscv64-linux-musl-objcopy && \
	ln -sf riscv64-unknown-elf-objdump riscv64-linux-musl-objdump
	@echo "✓ Toolchain symlinks created"

# ============================================================================
# Build Targets
# ============================================================================

# Setup ArceOS
ax_root:
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH"
	@if [ ! -f "$(AX_ROOT)/disk.img" ]; then \
		echo "Creating disk image..."; \
		dd if=/dev/zero of=$(AX_ROOT)/disk.img bs=1M count=64; \
	fi

# Generate default config
defconfig: ax_root
	@$(MAKE) -C $(AX_ROOT) A=$(A) EXTRA_CONFIG=$(EXTRA_CONFIG) defconfig

# Build kernel and app
build: defconfig install-toolchain
	@echo "Building Unfound OS for $(ARCH)..."
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	$(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) NET=$(NET) FEATURES=$(FEATURES) build
	@echo "✓ Build complete: $(OUT_ELF)"

# ============================================================================
# Run Targets
# ============================================================================

# Run in QEMU
run: build
	@echo "Running Unfound OS in QEMU..."
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	$(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) NET=$(NET) FEATURES=$(FEATURES) run

# Just run (without rebuild)
justrun: install-toolchain
	@echo "Running Unfound OS in QEMU (no rebuild)..."
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	$(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) NET=$(NET) FEATURES=$(FEATURES) justrun

# Debug with GDB
debug: build
	@echo "Starting GDB debug session..."
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	$(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) NET=$(NET) FEATURES=$(FEATURES) debug

# ============================================================================
# Test Targets
# ============================================================================

# Build unfound kernel (with integrated modules)
build-kernel:
	@echo "Building Unfound Kernel with UNotify/UCache..."
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	cd $(PWD) && cargo build --release --target $(TARGET)
	@echo "✓ Unfound kernel built"

# Run unfound kernel (which runs internal tests)
run-kernel: build-kernel
	@echo "Running Unfound Kernel..."
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	$(MAKE) -C $(AX_ROOT) A=$(PWD) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) FEATURES=$(FEATURES) justrun

# Run UNotify test (using ArceOS runtime)
test: A = $(APP_DIR)/unotify_test
test: build
	@echo "=========================================="
	@echo "  Running UNotify Test"
	@echo "=========================================="
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	timeout 30 $(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) FEATURES=$(FEATURES) justrun || true
	@echo ""
	@echo "Press Ctrl-A X to exit QEMU"

# Run with unfound kernel
test-unfound:
	@echo "=========================================="
	@echo "  Running Unfound Kernel Test"
	@echo "=========================================="
	@$(MAKE) run-kernel

# Run UCache test
test-cache: A = $(APP_DIR)/cache_test
test-cache: build
	@echo "=========================================="
	@echo "  Running UCache Performance Test"
	@echo "=========================================="
	@export PATH="$(TOOLCHAIN_DIR)/bin:$$PATH" && \
	timeout 30 $(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) LOG=$(LOG) \
		BLK=$(BLK) FEATURES=$(FEATURES) justrun || true

# Run all tests
test-all: test test-cache

# ============================================================================
# Development Targets
# ============================================================================

# Generate documentation
doc:
	@echo "Generating documentation..."
	@cargo doc --workspace --no-deps --open

# Run clippy
clippy: defconfig
	@echo "Running clippy..."
	@AX_CONFIG_PATH=$(PWD)/.axconfig.toml cargo clippy \
		--target $(TARGET) --all-features -- -D warnings \
		-A clippy::new_without_default

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt --all

# Check formatting
fmt-check:
	@echo "Checking code formatting..."
	@cargo fmt --all -- --check

# ============================================================================
# Clean Targets
# ============================================================================

clean:
	@echo "Cleaning build artifacts..."
	@$(MAKE) -C $(AX_ROOT) A=$(A) ARCH=$(ARCH) clean
	@cargo clean
	@rm -f $(OUT_ELF) $(OUT_BIN)
	@rm -f .axconfig.toml
	@echo "✓ Clean complete"

clean-all: clean
	@echo "Removing disk image..."
	@rm -f $(AX_ROOT)/disk.img

# ============================================================================
# Quick Commands
# ============================================================================

# Quick RISC-V run
rv: ARCH = riscv64
rv: run

# Quick x86_64 run
x86: ARCH = x86_64
x86: BLK = n
x86: run

# Quick test with trace logging
trace: LOG = trace
trace: test

# ============================================================================
# Info
# ============================================================================

info:
	@echo "Unfound OS Build Configuration"
	@echo "==============================="
	@echo "ARCH:           $(ARCH)"
	@echo "PLAT:           $(PLAT)"
	@echo "TARGET:         $(TARGET)"
	@echo "LOG:            $(LOG)"
	@echo "SMP:            $(SMP)"
	@echo "BLK:            $(BLK)"
	@echo "NET:            $(NET)"
	@echo "FEATURES:       $(FEATURES)"
	@echo "APP:            $(A)"
	@echo "AX_ROOT:        $(AX_ROOT)"
	@echo "TOOLCHAIN_DIR:  $(TOOLCHAIN_DIR)"
	@echo ""
	@echo "Modules:"
	@echo "  - UCache:  Page cache with LRU"
	@echo "  - UNotify: File event notification"
	@echo "  - UVFS:    Virtual filesystem"
