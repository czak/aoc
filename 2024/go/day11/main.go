package main

import (
	"fmt"
	"strconv"

	"czak.pl/aoc2024/aoc"
)

var (
	ex1   []int          = []int{0, 1, 10, 99, 999}
	ex2   []int          = []int{125, 17}
	input []int          = []int{2, 54, 992917, 5270417, 2514, 28561, 0, 990}
	cache map[[2]int]int = map[[2]int]int{}
)

func main() {
	part1 := aoc.Reduce(input, 0, total(25))
	part2 := aoc.Reduce(input, 0, total(75))

	fmt.Println(part1)
	fmt.Println(part2)
}

func total(times int) func(int, int) int {
	return func(total int, stone int) int {
		return total + blink(stone, times)
	}
}

// how many stones after times blinks starting from stone
func blink(stone int, times int) int {
	if q, ok := cache[[2]int{stone, times}]; ok {
		return q
	}

	if times == 0 {
		return 1
	}

	var res int
	if stone == 0 {
		res = blink(1, times-1)
	} else if s := strconv.Itoa(stone); len(s)%2 == 0 {
		n1 := aoc.Atoi(s[:len(s)/2])
		n2 := aoc.Atoi(s[len(s)/2:])
		res = blink(n1, times-1) + blink(n2, times-1)
	} else {
		res = blink(stone*2024, times-1)
	}

	cache[[2]int{stone, times}] = res
	return res
}
