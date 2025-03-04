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

	var startPos, startDir vec
	startPos.x, startPos.y, _ = g.Find('^')
	startDir.x, startDir.y = 0, -1

	pos := startPos
	dir := startDir

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

	count := 0

	for candidate := range visited {
		// place obstacle
		g[candidate.y][candidate.x] = '#'

		pos = startPos
		dir = startDir

		// track both position and dir to detect loops
		visited2 := map[[2]vec]bool{}

		for g.At(pos.x, pos.y) != ' ' {
			// have we been here before?
			if visited2[[2]vec{pos, dir}] {
				count++
				break
			}

			visited2[[2]vec{pos, dir}] = true

			if g.At(pos.x+dir.x, pos.y+dir.y) == '#' {
				dir = rotate(dir)
				continue
			}

			pos.x += dir.x
			pos.y += dir.y
		}

		// remove obstacle
		g[candidate.y][candidate.x] = '.'
	}

	fmt.Println(count)
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
