package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"

	"czak.pl/aoc2024/aoc"
)

const (
	ex1 = `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
	ex2 = `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
)

func main() {
	// input := ex2
	input := aoc.ReadAll(os.Stdin)

	part1(input)
	part2(input)
}

func part1(input string) {
	re := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)

	total := 0

	for _, op := range re.FindAllStringSubmatch(input, -1) {
		a, _ := strconv.Atoi(op[1])
		b, _ := strconv.Atoi(op[2])
		total += a * b
	}

	fmt.Println(total)
}

func part2(input string) {
	re := regexp.MustCompile(`(?:do\(\))|(?:don't\(\))|(?:mul\((\d{1,3}),(\d{1,3})\))`)

	total := 0
	enabled := true

	for _, op := range re.FindAllStringSubmatch(input, -1) {
		switch op[0] {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if enabled {
				a, _ := strconv.Atoi(op[1])
				b, _ := strconv.Atoi(op[2])
				total += a * b
			}
		}
	}

	fmt.Println(total)
}
