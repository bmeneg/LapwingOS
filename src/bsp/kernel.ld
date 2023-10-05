/* Output format for what we're planning to use as target: only
 * little-endian AArch64 */
OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")

/* The RPi4B firmware requires one of the following physical addrs for the
 * kernel image:
 *
 * 0x8000: 32 bits
 * 0x200000: 64 bits (un)compressed (gzip) image:
 *		bootloader decides based on the signature found at the beginning of
 *		the image
 *
 * Source:
 * https://www.raspberrypi.com/documentation/computers/config_txt.html#arm_64bit
 * https://www.raspberrypi.com/documentation/computers/config_txt.html#kernel_address
 */
kernel_load_addr = 0x200000;
ENTRY(kernel_load_addr)

/* Program headers (segments) defining how the executable must be loaded:
 * We want to load the "code" segment with RX permissions, meaning we need
 * to load it with PF_R and PF_X bits set. Considering:
 *
 * PF_X = 1
 * PF_W = 2
 * PF_R = 4
 *
 * For code segment:
 * PF_R + PF_X = 5
 * For data segment:
 * PF_R + PF_W = 6
 *
 * Source: https://refspecs.linuxfoundation.org/elf/elf.pdf
 */
PHDRS
{
	segment_code PT_LOAD FLAGS(5);
	segment_data PT_LOAD FLAGS(6);
}

SECTIONS
{
	/* AArch64 stack pointers are 128 bits aligned */
	.kernel_stack (NOLOAD) : ALIGN(16)
	{
		. += kernel_load_addr;
		/* Stack grows backwards, towards the beginning of RAM (0x00) */
		__kernel_stack_start = .;
	} :segment_data

	/* Code segment to hold actual kernel code */
	.text :
	{
		KEEP(*(.text._start));
		*(.text .text.*);
	} :segment_code
	/* Read-only data used in the kernel code */
	.rodata : {
		*(.rodata .rodata.*);
	} :segment_code

	/* Data segment used for heap and stack */
	.data : {
		*(.data .data.*)
	} :segment_data
	/* Data segment used for unitialized data (constants and static
	 * variables) that must be =0 at run time */
	.bss (NOLOAD) : ALIGN(16) {
		__bss_start = .;
		*(.bss .bss.*)
		*(COMMON)
		__bss_end = .;
	} :segment_data

	/* We don't care about these optional sections for now */
	/DISCARD/ : {
		*(.comment);
		*(.gnu*);
		*(.note*);
		*(.eh_frame*);
	}
}
