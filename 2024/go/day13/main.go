package main

import (
	"errors"
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

	part1 := 0
	part2 := 0

	for block := range strings.SplitSeq(input, "\n\n") {
		var ax, ay, bx, by, tx, ty int
		fmt.Sscanf(block, "Button A: X+%d, Y+%d\nButton B: X+%d, Y+%d\nPrize: X=%d, Y=%d", &ax, &ay, &bx, &by, &tx, &ty)

		// clear(memo)
		// cost := solve(vec{tx, ty}, vec{ax, ay}, vec{bx, by})

		cost1, err := linsolve(vec{tx, ty}, vec{ax, ay}, vec{bx, by})
		if err == nil {
			part1 += cost1
		}

		cost2, err := linsolve(vec{tx + 10000000000000, ty + 10000000000000}, vec{ax, ay}, vec{bx, by})
		if err == nil {
			part2 += cost2
		}

		fmt.Println("cost 1", cost1)
		fmt.Println("cost 2", cost2)
		fmt.Println()
	}

	fmt.Println(part1)
	fmt.Println(part2)
}

// t = x * a + y * b
func linsolve(t vec, a, b vec) (int, error) {
	m1 := b.x
	m2 := b.y

	a.x *= m2
	b.x *= m2
	t.x *= m2

	a.y *= m1
	b.y *= m1
	t.y *= m1

	fmt.Println(a.x, "*x +", b.x, "*y =", t.x)
	fmt.Println(a.y, "*x +", b.y, "*y =", t.y)

	dx := a.x - a.y
	dt := t.x - t.y

	fmt.Println(dx, " * x = ", dt)

	// is there an integer x?
	if dt%dx != 0 {
		return 0, errors.New("no solution")
	}

	x := dt / dx

	a.x *= x

	fmt.Println(a.x, "+", b.x, "*y =", t.x)

	dy := b.x
	dt = t.x - a.x

	// is there an integer y?
	if dt%dy != 0 {
		return 0, errors.New("no solution")
	}

	y := dt / dy

	fmt.Println(x, y)

	return 3*x + y, nil
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
