package main

import (
	"fmt"
	"os"
	"slices"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const ex1 = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`

type ab struct {
	a, b int
}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))
	parts := strings.Split(input, "\n\n")

	rules := map[ab]int{}
	updates := [][]int{}

	for line := range strings.SplitSeq(parts[0], "\n") {
		a := aoc.Atoi(line[0:2])
		b := aoc.Atoi(line[3:5])
		rules[ab{a, b}] = -1
		rules[ab{b, a}] = 1
	}

	for line := range strings.SplitSeq(parts[1], "\n") {
		updates = append(updates, aoc.Map(strings.Split(line, ","), aoc.Atoi))
	}

	part1(rules, updates)
}

func part1(rules map[ab]int, updates [][]int) {
	total := 0

	for _, update := range updates {
		sorted := slices.Clone(update)
		slices.SortFunc(sorted, func(a, b int) int {
			return rules[ab{a, b}]
		})

		if slices.Equal(update, sorted) {
			total += update[len(update)/2]
		}
	}

	fmt.Println(total)
}
