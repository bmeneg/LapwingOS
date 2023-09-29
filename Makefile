# SPDX-License-Identifier: BSD-3-Clause
#
# Copyright (c) 2023 Bruno Meneguele <bmeneg@heredoc.io>

# We only support RPI4, so fill all vars considering it
BSP = rpi4
BUILD_TARGET = aarch64-unknown-none-softfloat

ifdef $(RELEASE)
IS_RELEASE = --release
TARGET_PATH = release
else
TARGET_PATH = debug
endif

# Files to the end result
LD_SCRIPT_PATH = $(shell pwd)/src/bsp/rpi4
KERNEL_NAME = lapwing
KERNEL_LD_SCRIPT = kernel.ld
KERNEL_ELF = target/$(BUILD_TARGET)/$(TARGET_PATH)/$(KERNEL_NAME)
KERNEL_BINARY = target/$(BUILD_TARGET)/$(TARGET_PATH)/$(KERNEL_NAME).bin

# QEMU options for getting the kernel up and running
QEMU = qemu-system-aarch64
QEMU_MACHINE_TYPE = -M virt,highmem=off -smp 8 -m 2G -cpu cortex-a72
QEMU_ARGS = $(QEMU_MACHINE_TYPE) -d in_asm -display none

# Rust specific variables
RUSTCFLAGS = \
	$(IS_RELEASE) \
	-C target-cpu=cortex-a72 \
	-C link-arg=--library-path=$(LD_SCRIPT_PATH) \
	-C link-arg=--script=$(KERNEL_LD_SCRIPT)
RUSTC = cargo rustc 
OBJCOPY = cargo objcopy
OBJDUMP = cargo objdump

ifneq ($(shell which rustfilt),)
RUSTFILT = | rustfilt
endif

# Time for the targets
.PHONY: all qemu objdump clean

all: $(KERNEL_BINARY)

$(KERNEL_ELF):
	@echo "Compiling kernel ELF for $(BSP): $(KERNEL_ELF) ..."
	@$(RUSTC) -- $(RUSTCFLAGS)
	@echo "Done compilation"

$(KERNEL_BINARY): $(KERNEL_ELF)
	@echo "Generating kernel (stripped) binary: $(KERNEL_BINARY) ..."
	@$(OBJCOPY) --bin $(KERNEL_NAME) -- --strip-all -O binary $(KERNEL_BINARY)
	@echo "Done generation"

objdump: $(KERNEL_ELF)
	@echo "Launching Rust's objdump ..."
	@$(OBJDUMP) -- --disassemble --demangle --section .text $(KERNEL_ELF) $(RUSTFILT)

qemu: $(KERNEL_BINARY)
	@echo "Launching QEMU ..."
	@$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_BINARY)

clean:
	cargo clean
