package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const input = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

type Report []int

func main() {
	input := aoc.ReadAll(os.Stdin)

	reports := parse(input)

	part1(reports)
}

func parse(input string) []Report {
	var res []Report

	for line := range strings.Lines(input) {
		res = append(res, aoc.Map(strings.Fields(line), func(s string) int {
			n, err := strconv.Atoi(s)
			if err != nil {
				log.Fatal("not a number")
			}
			return n
		}))
	}

	return res
}

func part1(reports []Report) {
	count := 0

	for _, report := range reports {
		if isSafe(report) {
			count++
		}
	}

	fmt.Println(count)
}

func isSafe(report Report) bool {
	scores := map[int]int{
		-3: -1,
		-2: -1,
		-1: -1,
		1:  1,
		2:  1,
		3:  1,
	}

	a := report[0]
	b := report[1]
	prevScore := scores[b-a]

	for _, b = range report[1:] {
		score := scores[b-a]
		if score == 0 || score != prevScore {
			return false
		}

		a = b
		prevScore = score
	}

	return true
}
