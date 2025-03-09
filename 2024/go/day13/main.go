package main

import (
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex1 = `Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279`

// A - 3 tokens
// B - 1 token

type vec struct{ x, y int }

func (v vec) sub(other vec) vec {
	return vec{
		v.x - other.x,
		v.y - other.y,
	}
}

var memo map[vec]int = map[vec]int{}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	total := 0

	for block := range strings.SplitSeq(input, "\n\n") {
		var ax, ay, bx, by, tx, ty int
		fmt.Sscanf(block, "Button A: X+%d, Y+%d\nButton B: X+%d, Y+%d\nPrize: X=%d, Y=%d", &ax, &ay, &bx, &by, &tx, &ty)

		clear(memo)
		cost := solve(vec{tx, ty}, vec{ax, ay}, vec{bx, by})
		if cost < 1_000_000 {
			total += cost
		}
	}

	fmt.Println(total)
}

func solve(t vec, va, vb vec) int {
	if t == (vec{0, 0}) {
		return 0
	}
	if t.x < 0 || t.y < 0 {
		return 1_000_000
	}
	if m, ok := memo[t]; ok {
		return m
	}
	best := 1_000_000
	a := solve(t.sub(va), va, vb) + 3
	if a < best {
		best = a
	}
	b := solve(t.sub(vb), va, vb) + 1
	if b < best {
		best = b
	}
	memo[t] = best
	return best
}
