.section .text

_start:

	la a1, msg
	addi a0, zero, 1

	addi a2, zero, 9

	addi a7, zero, 64
	ecall

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

other:
	nop

.section .data
msg:
	.ascii "Holaatodos"
