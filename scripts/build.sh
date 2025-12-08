#!/bin/bash
# 构建脚本

set -e

ARCH=${ARCH:-riscv64}
PLATFORM=${PLATFORM:-qemu-virt-riscv}

echo "Building Unfound OS for $ARCH..."

# 检查Rust工具链
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust toolchain not found"
    exit 1
fi

# 添加目标架构
rustup target add ${ARCH}-unknown-none

# 构建内核
cargo build --release --target ${ARCH}-unknown-none

echo "Build completed successfully!"
echo "Output: target/${ARCH}-unknown-none/release/unfound"
