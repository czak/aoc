package main

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex = `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`

type vec struct {
	x, y int
}

func main() {
	// input := ex
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	g := aoc.ParseGrid(input)

	var pos, dir vec
	pos.x, pos.y, _ = g.Find('^')
	dir.x, dir.y = 0, -1

	visited := map[vec]bool{}

	for g.At(pos.x, pos.y) != ' ' {
		visited[pos] = true

		if g.At(pos.x+dir.x, pos.y+dir.y) == '#' {
			dir = rotate(dir)
			continue
		}

		pos.x += dir.x
		pos.y += dir.y
	}

	fmt.Println(len(visited))
}

func rotate(dir vec) vec {
	switch dir {
	// N -> E
	case vec{0, -1}:
		return vec{1, 0}

	// E -> S
	case vec{1, 0}:
		return vec{0, 1}

	// S -> W
	case vec{0, 1}:
		return vec{-1, 0}

	// W -> N
	case vec{-1, 0}:
		return vec{0, -1}

	default:
		panic(errors.New("bad dir"))
	}
}
