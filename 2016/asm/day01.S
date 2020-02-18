# vim: set sw=4 ts=4:
.section .text
.globl main
main:
    addi sp, sp, -4
    sw ra, 4(sp)

    la a0, msg
    call puts

    lw ra, 4(sp)
    addi sp, sp, 4
    ret

.section .rodata
msg:
    .string "Hello, World!"
