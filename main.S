section .text

_start:
	# a0 = 0 exit code
	# a7 = 93 syscall number 
	addi a0, zero, 0
	addi a7, zero, 0
	addi a2, zero, 90
	addi a1, zero, 3

	add a7, a2, a1

	# ecall = system call
	ecall

other:
	nop 
