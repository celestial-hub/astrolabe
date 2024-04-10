.data
str_0: .asciiz "done!"

	.text
	.global main
main:
	li $t0, 0
start:
	addi $t0, $t0, 1
	blt $t0, 10, start
	j done
done:
	la $t1, str_0
	li $v0, 4
	move $a0, $t1
	syscall
