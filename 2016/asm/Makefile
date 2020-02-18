%: %.S
	riscv32-unknown-elf-gcc -march=rv32i -g $^ -o $@ -nostartfiles

.PHONY: run
run: $(DAY)
	qemu-riscv32 $(DAY)

.PHONY: debug
debug: $(DAY)
	qemu-riscv32 -g 1234 $(DAY) &
	riscv32-unknown-elf-gdb $(DAY) \
		-ex 'target remote :1234' \
		-ex 'dashboard registers'
