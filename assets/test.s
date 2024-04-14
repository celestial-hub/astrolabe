.data
__buffer_1: .space 50

	.text
	.global main
main:
	li $v0, 8
	la $a0, buffer
	li $a1, 50
	syscall
	move $t0, $v0
	li $v0, 4
	move $a0, $t0
	syscall
