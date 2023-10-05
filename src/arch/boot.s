.section .text._start, "ax"
.equ MPIDR_COREID_MASK, 0b11
.global _start

_start:
	// run on CPU0 only
	mrs x0, mpidr_el1
	ldr x1, =MPIDR_COREID_MASK
	and x0, x0, x1
	cbz x0, .L_mem_init

.L_loop_forever:
	wfe
	b .L_loop_forever

.L_mem_init:
	// set the stack pointer to right after our _start code
	ldr x0, =__kernel_stack_start
	mov sp, x0
	// set bss and zero it 16 bytes at time
	ldr x0, =__bss_start
	ldr x1, =__bss_end
.L_bss_zero_loop:
	cmp x1, x0
	b.eq .L_main
	stp xzr, xzr, [x0], #16
	b .L_bss_zero_loop

.L_main:
	b _start_kernel

.size _start, . - _start
.type _start, function
