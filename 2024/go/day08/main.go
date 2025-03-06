package main

import (
	"fmt"
	"iter"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex1 = `............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............`

type (
	vec  [2]int
	pair [2]vec
)

func (v vec) sub(other vec) vec {
	dx := v[0] - other[0]
	dy := v[1] - other[1]

	return vec{dx, dy}
}

func (v vec) within(xmax, ymax int) bool {
	if v[0] < 0 || v[0] > xmax || v[1] < 0 || v[1] > ymax {
		return false
	}
	return true
}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))
	g := aoc.ParseGrid(input)

	xmax := len(g[0]) - 1
	ymax := len(g) - 1

	antennas := map[rune][]vec{}

	for y, row := range g {
		for x, freq := range row {
			if freq == '.' {
				continue
			}
			antennas[freq] = append(antennas[freq], vec{x, y})
		}
	}

	antinodes := map[vec]bool{}

	for _, locations := range antennas {
		for pair := range pairs(locations) {
			a := pair[0]
			b := pair[1]
			diff := b.sub(a)

			u := a.sub(diff)
			v := b.sub(vec{0, 0}.sub(diff))

			antinodes[u] = true
			antinodes[v] = true
		}
	}

	count := 0

	for n := range antinodes {
		if n.within(xmax, ymax) {
			count++
		}
	}

	fmt.Println(count)
}

func pairs(locations []vec) iter.Seq[pair] {
	seen := map[pair]bool{}

	return func(yield func(pair) bool) {
		for _, a := range locations {
			for _, b := range locations {
				if a == b {
					continue
				}

				ab := pair{a, b}
				ba := pair{b, a}

				if seen[ba] {
					continue
				}

				seen[ab] = true

				if !yield(ab) {
					return
				}
			}
		}
	}
}
