.section .text

_start:

	#	la a1, msg
	addi a0, zero, 1

	addi a2, zero, 5

	addi a7, zero, 64
	ecall

	auipc a7, 2
	lui a7, 74565

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

other:
	nop

.section .data
msg:
	.ascii "Hola"
