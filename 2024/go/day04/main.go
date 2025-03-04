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

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	var g Grid
	for row := range strings.SplitSeq(input, "\n") {
		g = append(g, []rune(row))
	}

	count1 := 0
	count2 := 0

	for y := range len(g) {
		for x := range len(g[0]) {
			count1 += find1(g, x, y)
			count2 += find2(g, x, y)
		}
	}

	fmt.Println(count1)
	fmt.Println(count2)
}

func find1(g Grid, x, y int) int {
	count := 0

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

	if g.at(x, y) != 'X' {
		return 0
	}

	for _, d := range deltas {
		dx, dy := d[0], d[1]
		if g.at(x+dx, y+dy) == 'M' && g.at(x+2*dx, y+2*dy) == 'A' && g.at(x+3*dx, y+3*dy) == 'S' {
			count++
		}
	}

	return count
}

func find2(g Grid, x, y int) int {
	if g.at(x, y) != 'A' {
		return 0
	}

	d1 := map[rune]bool{
		g.at(x-1, y-1): true,
		g.at(x+1, y+1): true,
	}
	d2 := map[rune]bool{
		g.at(x-1, y+1): true,
		g.at(x+1, y-1): true,
	}

	if d1['M'] && d1['S'] && d2['M'] && d2['S'] {
		return 1
	}

	return 0
}
