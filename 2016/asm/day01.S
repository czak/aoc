# vim: set sw=4 ts=4:
.section .text
.globl main
main:
    addi sp, sp, -4
    sw ra, 4(sp)

    # current char: s0
    la s0, input

    li s1, 0    # x
    li s2, 0    # y
    li s3, 0    # dir (0=n 1=e 2=s 3=w)

    li s4, 'R'
    li s5, 'L'

mainloop:
    # decode turn ('R' or 'L')
    lb t0, 0(s0)
    addi s0, s0, 1
    beq t0, s4, turn_right
    # turn left
    addi s3, s3, -1
    j turn_done
turn_right:
    addi s3, s3, 1
turn_done:
    andi s3, s3, 3      # % 4

    # decode number of steps
    li t0, 0
    li t2, 10
next_digit:
    lb t1, 0(s0)
    addi t1, t1, -'0'
    bltu t2, t1, steps_done
    mul t0, t0, t2
    add t0, t0, t1
    addi s0, s0, 1
    j next_digit
steps_done:

    # s3 contains direction
    # t0 contains number of steps
    li t1, 0
    beq s3, t1, north
    li t1, 1
    beq s3, t1, east
    li t1, 2
    beq s3, t1, south
    li t1, 3
    beq s3, t1, west

north:
    add s2, s2, t0
    j 1f
east:
    add s1, s1, t0
    j 1f
south:
    sub s2, s2, t0
    j 1f
west:
    sub s1, s1, t0
    j 1f

1:
    # last char was either a comma or '\0'
    lb t0, 0(s0)
    beq t0, zero, done
    addi s0, s0, 2
    j mainloop

done:
    # abs(s1) + abs(s2)
    blt zero, s1, 1f
    sub s1, zero, s1
1:
    blt zero, s2, 2f
    sub s2, zero, s2
2:

    la a0, fmt
    add a1, s1, s2
    call printf

    # return 0
    li a0, 0

    lw ra, 4(sp)
    addi sp, sp, 4
    ret

.section .rodata
input:
    .string "R4, R1, L2, R1, L1, L1, R1, L5, R1, R5, L2, R3, L3, L4, R4, R4, R3, L5, L1, R5, R3, L4, R1, R5, L1, R3, L2, R3, R1, L4, L1, R1, L1, L5, R1, L2, R2, L3, L5, R1, R5, L1, R188, L3, R2, R52, R5, L3, R79, L1, R5, R186, R2, R1, L3, L5, L2, R2, R4, R5, R5, L5, L4, R5, R3, L4, R4, L4, L4, R5, L4, L3, L1, L4, R1, R2, L5, R3, L4, R3, L3, L5, R1, R1, L3, R2, R1, R2, R2, L4, R5, R1, R3, R2, L2, L2, L1, R2, L1, L3, R5, R1, R4, R5, R2, R2, R4, R4, R1, L3, R4, L2, R2, R1, R3, L5, R5, R2, R5, L1, R2, R4, L1, R5, L3, L3, R1, L4, R2, L2, R1, L1, R4, R3, L2, L3, R3, L2, R1, L4, R5, L1, R5, L2, L1, L5, L2, L5, L2, L4, L2, R3"

fmt:
    .string "Part 1: %d\n"
