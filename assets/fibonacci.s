  .data
prompt: .asciiz "Enter the value of n: "

  .text
  .global main
main:
    ; print prompt
    li $v0, 0x4
    la $a0, prompt
    syscall

    ; read n from user
    li $v0, 0x5
    syscall
    move $a0, $v0 ; move the value to $a0

    ; call fibonacci function
    jal fibonacci

    ; print the result
    li $v0, 0x1
    syscall

    ; exit
    li $v0, 0xA
    syscall

fibonacci:
    ; base case: if n == 0, return 0
    beq $a0, 0, return_zero

    ; base case: if n == 1, return 1
    li $t0, 1
    beq $a0, $t0, return_one

    ; recursive case: f(n) = f(n-1) + f(n-2)
    ; calculate f(n-1)
    sub $a0, $a0, 1
    jal fibonacci
    move $t1, $v0

    ; calculate f(n-2)
    sub $a0, $a0, 1
    jal fibonacci
    add $v0, $v0, $t1
    jr $ra

return_zero:
    move $v0, $zero
    jr $ra

return_one:
    move $v0, $t0
