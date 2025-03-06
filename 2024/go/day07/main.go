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

type operator func(int, int) int

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

	count(equations, []operator{add, mul})
	count(equations, []operator{add, mul, concat})
}

func count(equations []equation, ops []operator) {
	total := 0

	for _, e := range equations {
		if valid(e, ops) {
			total += e.test
		}
	}

	fmt.Println(total)
}

func valid(e equation, ops []operator) bool {
	if len(e.values) == 2 {
		for _, op := range ops {
			if e.test == op(e.values[0], e.values[1]) {
				return true
			}
		}
		return false
	}

	for _, op := range ops {
		if valid(equation{e.test, append([]int{op(e.values[0], e.values[1])}, e.values[2:]...)}, ops) {
			return true
		}
	}
	return false
}

func add(a, b int) int    { return a + b }
func mul(a, b int) int    { return a * b }
func concat(a, b int) int { return aoc.Atoi(fmt.Sprintf("%d%d", a, b)) }
