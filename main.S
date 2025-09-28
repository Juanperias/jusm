.section .text

.globl _start

_start:
	addi a0, zero, 1
	la a1, test
	addi a2, zero, 6
	la a1, msg
	addi a7, zero, 64

	ecall

	addi sp, sp, 16
	addi a1, zero, 0
	sb a1, 0(sp)
	addi sp, sp, -16

	addi a0, zero, 1
	addi a2, zero, 9
	la a1, msg2

	addi a7, zero, 64
	ecall

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

test:
   la a0, msg
   nop

.global other
.weak other

other:
	nop

.section .data
msg:
	.ascii "Hello"

msg2: .ascii "Hello2"
