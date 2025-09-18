.section .text

.globl _start

_start:
	addi a0, zero, 1

	addi a2, zero, 6
	la a1, msg


	addi a7, zero, 64

	ecall

	addi a0, zero, 1
	addi a2, zero, 9
	la a1, msg2

	addi a7, zero, 64
	ecall

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

other:
	nop

.section .data
msg:
	.ascii "Hello"

msg2: .ascii "Hello2"
