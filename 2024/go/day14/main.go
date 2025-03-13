package main

import (
	"fmt"
	"os"
	"slices"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex1 = `p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3`

const ex2 = `p=2,4 v=2,-3`

const (
	W = 101
	H = 103
)

type vec struct {
	x, y int
}

type robot struct {
	p, v vec
}

func (r *robot) simulate(n int) {
	r.p.x = ((r.p.x+n*r.v.x)%W + W) % W
	r.p.y = ((r.p.y+n*r.v.y)%H + H) % H
}

func (v vec) quadrant() int {
	if v.x < W/2 {
		if v.y < H/2 {
			return 0
		} else if v.y > H/2 {
			return 1
		}
	} else if v.x > W/2 {
		if v.y < H/2 {
			return 2
		} else if v.y > H/2 {
			return 3
		}
	}
	return 4
}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	robots := []robot{}

	for s := range strings.SplitSeq(input, "\n") {
		var r robot
		fmt.Sscanf(s, "p=%d,%d v=%d,%d", &r.p.x, &r.p.y, &r.v.x, &r.v.y)
		robots = append(robots, r)
	}

	part1(slices.Clone(robots))
	part2(robots)
}

func part1(robots []robot) {
	counts := [5]int{}
	for i := range robots {
		r := &robots[i]
		r.simulate(100)
		q := r.p.quadrant()
		counts[q]++
	}

	total := counts[0] * counts[1] * counts[2] * counts[3]

	fmt.Println(total)
}

func part2(robots []robot) {
	for i := range robots {
		r := &robots[i]
		r.simulate(40)
	}

	for s := range 200 {
		fmt.Println("sec:", 40+s*103)
		dump(robots)

		for i := range robots {
			r := &robots[i]
			r.simulate(103)
		}
	}
}

func dump(robots []robot) {
	grid := [H][W]int{}
	for i := range robots {
		r := &robots[i]
		grid[r.p.y][r.p.x]++
	}
	for y := range H {
		for x := range W {
			fmt.Printf("%c", ch(grid[y][x]))
		}
		fmt.Println()
	}
}

func ch(n int) rune {
	switch n {
	case 0:
		return ' '
	default:
		return '#'
	}
}
