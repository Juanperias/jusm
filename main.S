.section .text

_start:
	addi sp, sp, -5
	addi t3, zero, 'H'
	sb t3, 0(sp)

	addi t3, zero, 'o'
	sb t3, 1(sp)

	addi t3, zero, 'l'
	sb t3, 2(sp)

	addi t3, zero, 'a'
	sb t3, 3(sp)

	addi t3, zero, '\n'
	sb t3, 4(sp)


	addi a0, zero, 1
	addi a1, sp, 0
	addi a2, zero, 5
	addi a7, zero, 64
	ecall

	addi a0, zero, 0
	addi a7, zero, 93
	ecall

other:
	nop 
