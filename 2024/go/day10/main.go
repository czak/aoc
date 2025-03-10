package main

import (
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

// score 1
const ex1 = `0123
1234
8765
9876`

// score 36
const ex2 = `89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732`

type (
	grid  [][]int
	vec   [2]int
	route [10]vec
)

func (g grid) at(x, y int) int {
	if x < 0 || x >= len(g[0]) || y < 0 || y >= len(g) {
		return -1 // cliff
	}
	return g[y][x]
}

func main() {
	// input := ex2
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))
	g := parse(input)

	totalPeaks := 0
	totalRoutes := 0

	for y := range len(g) {
		for x := range len(g[0]) {
			if g.at(x, y) != 0 {
				continue
			}

			peaks := map[vec]bool{}
			routes := map[route]bool{}

			climb(g, x, y, route{}, peaks, routes)

			totalPeaks += len(peaks)
			totalRoutes += len(routes)
		}
	}

	fmt.Println(totalPeaks)
	fmt.Println(totalRoutes)
}

func climb(g grid, x, y int, r route, peaks map[vec]bool, routes map[route]bool) {
	level := g.at(x, y)

	r[level] = vec{x, y}

	if level == 9 {
		peaks[vec{x, y}] = true
		routes[r] = true
		return
	}

	if g.at(x, y-1) == level+1 {
		climb(g, x, y-1, r, peaks, routes)
	}
	if g.at(x+1, y) == level+1 {
		climb(g, x+1, y, r, peaks, routes)
	}
	if g.at(x, y+1) == level+1 {
		climb(g, x, y+1, r, peaks, routes)
	}
	if g.at(x-1, y) == level+1 {
		climb(g, x-1, y, r, peaks, routes)
	}
}

func parse(input string) grid {
	var g grid
	for row := range strings.SplitSeq(input, "\n") {
		g = append(g, aoc.Map([]rune(row), func(r rune) int {
			return int(r - '0')
		}))
	}
	return g
}
