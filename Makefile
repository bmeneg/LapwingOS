# We only support RPI4, so fill all vars considering it
BSP = rpi4
BUILD_TARGET = aarch64-unknown-none-softfloat

ifdef $(RELEASE)
IS_RELEASE = --release
TARGET_PATH = release
else
TARGET_PATH = debug
endif

# File names used for target execution steps and also documentation
KERNEL_NAME = lapwing
KERNEL_ELF = target/$(BUILD_TARGET)/$(TARGET_PATH)/$(KERNEL_NAME)
KERNEL_BINARY = target/$(BUILD_TARGET)/$(TARGET_PATH)/$(KERNEL_NAME).bin

# QEMU options for getting the kernel up and running
QEMU = qemu-system-aarch64
QEMU_MACHINE_TYPE = -M virt,highmem=off -smp 8 -m 2G -cpu cortex-a72
QEMU_ARGS = $(QEMU_MACHINE_TYPE) -d in_asm -display none

# Rust executables for different targets
# NOTE: since we're using cargo-binutils crate, every `cargo <tool>` call will first execute `cargo build`, meaning we
# need to place rust compiler flags into .cargo/config.toml to avoid multiple compilations (on every cargo call). Once
# cargo realizes there is no incremental compilation to perform, it'll successfully finish the `build` step.
RUSTC = cargo rustc 
OBJCOPY = cargo objcopy
OBJDUMP = cargo objdump
READOBJ = cargo readobj

ifneq ($(shell which rustfilt),)
RUSTFILT = | rustfilt
endif

# Time for the targets
.PHONY: all qemu objdump readobj clean

all: $(KERNEL_BINARY)

$(KERNEL_ELF):
	@echo "Compiling kernel ELF for $(BSP): $(KERNEL_ELF) ..."
	$(RUSTC) --verbose
	@echo "Done compilation"

$(KERNEL_BINARY): $(KERNEL_ELF)
	@echo "Generating kernel (stripped) binary: $(KERNEL_BINARY) ..."
	$(OBJCOPY) -- --strip-all -O binary $(KERNEL_BINARY)
	@echo "Done generation"

objdump: $(KERNEL_ELF)
	@echo "Launching Rust's objdump ..."
	$(OBJDUMP) -- --disassemble --demangle --section .text $(RUSTFILT)

readobj: $(KERNEL_ELF)
	@echo "Launching Rust's readobj ..."
	# use GNU style because we're already used to it from older days
	$(READOBJ) -- --elf-output-style=GNU --all

qemu: $(KERNEL_BINARY)
	@echo "Launching QEMU ..."
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_BINARY)

clean:
	cargo clean
