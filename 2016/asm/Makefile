%: %.S
	riscv32-unknown-elf-gcc -march=rv32im -g $^ -o $@

.PHONY: run
run: $(DAY)
	qemu-riscv32 $(DAY)

.PHONY: debug
debug: $(DAY)
	qemu-riscv32 -g 1234 $(DAY) &
	riscv32-unknown-elf-gdb $(DAY) \
		-ex 'target remote :1234' \
		-ex 'dashboard registers' \
		-ex 'dashboard source -style height 40' \
		-ex 'break main' \
		-ex 'continue'
