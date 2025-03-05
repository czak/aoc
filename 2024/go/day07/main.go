package main

import (
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex1 = `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`

type equation struct {
	test   int
	values []int
}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	equations := []equation{}

	for line := range strings.SplitSeq(input, "\n") {
		parts := strings.Split(line, ": ")
		equations = append(equations, equation{
			test:   aoc.Atoi(parts[0]),
			values: aoc.Map(strings.Split(parts[1], " "), aoc.Atoi),
		})
	}

	total := 0
	for _, e := range equations {
		if valid(e) {
			total += e.test
		}
	}

	fmt.Println(total)
}

func valid(e equation) bool {
	// only 2 left, just check + or * or ||
	if len(e.values) == 2 {
		return e.test == e.values[0]+e.values[1] || e.test == e.values[0]*e.values[1] || e.test == combine(e.values[0], e.values[1])
	}

	return valid(equation{
		test:   e.test,
		values: append([]int{e.values[0] + e.values[1]}, e.values[2:]...),
	}) || valid(equation{
		test:   e.test,
		values: append([]int{e.values[0] * e.values[1]}, e.values[2:]...),
	}) || valid(equation{
		test:   e.test,
		values: append([]int{combine(e.values[0], e.values[1])}, e.values[2:]...),
	})
}

func combine(a, b int) int {
	return aoc.Atoi(fmt.Sprintf("%d%d", a, b))
}
