.section .text

_start:
	# a0 = 0 exit code
	# a7 = 93 syscall number 
	addi a0, zero, 20
	xor a0, a0, a0
	addi a7, zero, 93
	# ecall = system call
	ecall

other:
	nop 
