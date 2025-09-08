.section .text

_start:
	addi sp, sp, -32
	addi t3, zero, 72
	sb t3, 0(sp)

	addi a0, zero, 1
	addi a1, sp, 0
	addi a2, zero, 1
	addi a7, zero, 64
	ecall

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

other:
	nop 
