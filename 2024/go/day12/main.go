package main

import (
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

// 140
const ex1 = `AAAA
BBCD
BBCC
EEEC`

// 772 (756 + 4 + 4 + 4 + 4)
const ex2 = `OOOOO
OXOXO
OOOOO
OXOXO
OOOOO`

// 1930
const ex3 = `RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`

type vec [2]int

func main() {
	// input := ex3
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	g := partition(aoc.ParseGrid(input))

	areas := map[int]int{}
	perimeters := map[int]int{}

	// rows
	for y := range g {
		for x := range g[0] {
			r := g[y][x]

			areas[r]++

			// left edge
			if x == 0 {
				perimeters[r]++
				continue
			}

			// right edge
			if x == len(g[0])-1 {
				perimeters[r]++
			}

			// between plots
			prev := g[y][x-1]
			if r != prev {
				perimeters[prev]++
				perimeters[r]++
			}
		}
	}

	// columns
	for x := range g[0] {
		for y := range g[0] {
			r := g[y][x]

			// top edge
			if y == 0 {
				perimeters[r]++
				continue
			}

			// bottom edge
			if y == len(g)-1 {
				perimeters[r]++
			}

			// between plots
			prev := g[y-1][x]
			if r != prev {
				perimeters[prev]++
				perimeters[r]++
			}
		}
	}

	total := 0
	for k, area := range areas {
		perimeter := perimeters[k]
		total += area * perimeter
	}

	fmt.Println(areas)
	fmt.Println(perimeters)

	fmt.Println(total)
}

func partition(g aoc.Grid) [][]int {
	res := make([][]int, len(g))
	for y := range res {
		res[y] = make([]int, len(g[0]))
	}

	pid := 1
	pmap := map[vec]int{}
	for y := range g {
		for x := range g[0] {
			if pmap[vec{x, y}] != 0 {
				continue
			}

			seen := map[vec]bool{}
			fill(g, x, y, seen)

			for v := range seen {
				pmap[v] = pid
			}

			pid++
		}
	}

	for v, id := range pmap {
		res[v[1]][v[0]] = id
	}

	return res
}

func fill(g aoc.Grid, x, y int, seen map[vec]bool) {
	if seen[vec{x, y}] {
		return
	}

	seen[vec{x, y}] = true

	r := g.At(x, y)

	if g.At(x+1, y) == r {
		fill(g, x+1, y, seen)
	}
	if g.At(x-1, y) == r {
		fill(g, x-1, y, seen)
	}
	if g.At(x, y+1) == r {
		fill(g, x, y+1, seen)
	}
	if g.At(x, y-1) == r {
		fill(g, x, y-1, seen)
	}
}
