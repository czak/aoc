package main

import (
	"fmt"
	"os"
	"slices"
	"strings"

	"czak.pl/aoc2024/aoc"
)

var example = strings.NewReader(`3   4
4   3
2   5
1   3
3   9
3   3`)

func main() {
	var l1, l2 []int

	for line := range aoc.Lines(os.Stdin) {
		var a, b int
		fmt.Sscan(line, &a, &b)
		l1 = append(l1, a)
		l2 = append(l2, b)
	}

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
