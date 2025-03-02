package main

import (
	"fmt"
	"os"
	"slices"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const input = `3   4
4   3
2   5
1   3
3   9
3   3`

func main() {
	var l1, l2 []int

	input := aoc.ReadAll(os.Stdin)

	for line := range strings.Lines(input) {
		var a, b int
		fmt.Sscan(line, &a, &b)
		l1 = append(l1, a)
		l2 = append(l2, b)
	}

	part1(l1, l2)
	part2(l1, l2)
}

func part1(l1, l2 []int) {
	slices.Sort(l1)
	slices.Sort(l2)

	total := 0

	for i := range len(l1) {
		dist := l1[i] - l2[i]
		if dist < 0 {
			total -= dist
		} else {
			total += dist
		}
	}

	fmt.Println(total)
}

func part2(l1, l2 []int) {
	counts := map[int]int{}

	for _, n := range l2 {
		counts[n]++
	}

	total := 0

	for _, n := range l1 {
		total += n * counts[n]
	}

	fmt.Println(total)
}
