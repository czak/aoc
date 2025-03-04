package aoc

import (
	"fmt"
	"strings"
)

type Grid [][]rune

func ParseGrid(input string) Grid {
	var g Grid
	for row := range strings.SplitSeq(input, "\n") {
		g = append(g, []rune(row))
	}
	return g
}

func (g Grid) At(x, y int) rune {
	if x < 0 || y < 0 || x >= len(g[0]) || y >= len(g) {
		return ' '
	}
	return g[y][x]
}

func (g Grid) Find(c rune) (int, int, error) {
	for y := range len(g) {
		for x := range len(g[0]) {
			if g[y][x] == c {
				return x, y, nil
			}
		}
	}
	return 0, 0, fmt.Errorf("%c not found", c)
}
