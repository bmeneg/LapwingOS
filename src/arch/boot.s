.section .text._start

.global _start
_start:
.loop_forever:
	wfe
	b .loop_forever

.size _start, . - _start
.type _start, function
