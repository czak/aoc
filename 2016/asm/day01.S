.section .text
.globl _start
_start:
    li a0, 1                    # stdout
1:  auipc a1, %pcrel_hi(msg)    # load msg(hi)
    addi a1, a1, %pcrel_lo(1b)  # load msg(lo)
    la a1, msg
    li a2, 14                   # length
    li a7, 64                   # _NR_sys_write
    ecall                       # system call

    li a0, 0
    li a7, 93                   # _NR_sys_exit
    ecall                       # system call

.section .rodata
msg:
    .string "Hello, World!\n"
