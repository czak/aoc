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
	part2(reports)
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
	fmt.Println(aoc.Count(reports, isSafe))
}

func part2(reports []Report) {
	count := aoc.Count(reports, func(report Report) bool {
		if isSafe(report) {
			return true
		}
		for i := range len(report) {
			modified := Report{}
			modified = append(modified, report[:i]...)
			modified = append(modified, report[i+1:]...)

			if isSafe(modified) {
				return true
			}
		}
		return false
	})

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

	total := 0

	for i := range len(report) - 1 {
		total += scores[report[i+1]-report[i]]
	}

	return total == -len(report)+1 || total == len(report)-1
}
