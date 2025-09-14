.section .text

_start:


	addi a0, zero, 1

	addi a2, zero, 4

	addi a7, zero, 64
	la a1, msg
	ecall

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

other:
	nop

.section .data
msg:
	.ascii "Hello"
