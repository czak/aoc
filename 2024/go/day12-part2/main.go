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

// 236
const ex4 = `EEEEE
EXXXX
EEEEE
EXXXX
EEEEE`

type (
	vec  [2]int
	grid [][]int
)

func (g grid) at(x, y int) int {
	if x < 0 || x >= len(g[0]) || y < 0 || y >= len(g) {
		return 0
	}
	return g[y][x]
}

func (g grid) row(y int) []int {
	res := make([]int, 0)
	for x := range g[0] {
		res = append(res, g.at(x, y))
	}
	return res
}

func (g grid) col(x int) []int {
	res := make([]int, 0)
	for y := range g {
		res = append(res, g.at(x, y))
	}
	return res
}

func zip(a, b []int) [][2]int {
	res := make([][2]int, 0)
	for i := range a {
		res = append(res, [2]int{a[i], b[i]})
	}
	return res
}

func main() {
	// input := ex3
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	g := partition(aoc.ParseGrid(input))

	areas := map[int]int{}
	perimeters := map[int]int{}

	// areas
	for y := range g {
		for x := range g[0] {
			areas[g[y][x]]++
		}
	}

	// perimeters - rows
	for y := range len(g) + 1 {
		pairs := zip(g.row(y-1), g.row(y))
		// pairs = slices.Compact(pairs)
		var ba, bb int
		for _, pair := range pairs {
			a := pair[0]
			b := pair[1]
			if a != b {
				if a != ba {
					perimeters[a]++
					ba = a
				}
				if b != bb {
					perimeters[b]++
					bb = b
				}
			} else {
				ba = -1
				bb = -1
			}
		}
	}

	// perimeters - cols
	for x := range len(g[0]) + 1 {
		pairs := zip(g.col(x-1), g.col(x))
		var ba, bb int
		for _, pair := range pairs {
			a := pair[0]
			b := pair[1]
			if a != b {
				if a != ba {
					perimeters[a]++
					ba = a
				}
				if b != bb {
					perimeters[b]++
					bb = b
				}
			} else {
				ba = -1
				bb = -1
			}
		}
	}

	// NOTE: perimeters contains key 0 too (outer border)
	total := 0
	for k, area := range areas {
		perimeter := perimeters[k]
		total += area * perimeter
	}

	// fmt.Println(areas)
	// fmt.Println(perimeters)

	fmt.Println(total)
}

func partition(g aoc.Grid) grid {
	res := make(grid, len(g))
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
