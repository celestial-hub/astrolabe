  .data
prompt: .asciiz "Enter the value of n: "

	.text
	.global main
main:
  ; Add 1 + 2
	li $t0, 1
	li $t1, 2
	add $t2, $t0, $t1

  ; Print prompt
  li $v0, 4
  la $a0, prompt
  syscall

  ; Print added value
  li $v0, 1
  move $a0, $t2
  syscall

  ; Exit
  li $v0, 0xA
  syscall
