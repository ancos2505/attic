# riscv-hello-uart
Minimal bare-metal RISC-V assembly code with UART output for execution in QEMU

## Requirements
### Tools:
- riscv64-unknown-elf-gcc
- riscv64-unknown-elf-ld
- riscv64-unknown-elf-objcopy

### Building:
Make

### Execution:
qemu-system-riscv64

## Building
make all

## Running
make run

## Dump Device Tree Table from Qemu
qemu-system-riscv32 -cpu rv32 -bios none -M virt -machine dumpdtb=riscv32-virt.dtb

## Convert .dtb (binary format) to .dts (string format)
dtc -I dtb -O dts -o riscv32-virt.dts riscv32-virt.dtb
