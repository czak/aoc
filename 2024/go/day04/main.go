package main

import (
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex1 = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`

type Grid [][]rune

func (g Grid) at(x, y int) rune {
	if x < 0 || y < 0 || x >= len(g[0]) || y >= len(g) {
		return ' '
	}
	return g[y][x]
}

func (g Grid) mas(x, y int, dx, dy int) bool {
	return g.at(x+dx, y+dy) == 'M' && g.at(x+2*dx, y+2*dy) == 'A' && g.at(x+3*dx, y+3*dy) == 'S'
}

func main() {
	// input := ex1
	input := aoc.ReadAll(os.Stdin)

	var g Grid
	for row := range strings.SplitSeq(input, "\n") {
		if len(row) == 0 {
			break
		}
		g = append(g, []rune(row))
	}

	deltas := [][2]int{
		{-1, -1},
		{-1, 0},
		{-1, 1},
		{0, -1},
		{0, 1},
		{1, -1},
		{1, 0},
		{1, 1},
	}

	count := 0

	for y := range len(g) {
		for x := range len(g[0]) {
			if g.at(x, y) != 'X' {
				continue
			}

			for _, d := range deltas {
				if g.mas(x, y, d[0], d[1]) {
					count++
				}
			}
		}
	}

	fmt.Println(count)
}
