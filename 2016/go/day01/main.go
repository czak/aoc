package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"strconv"
)

const (
	NORTH = 0
	EAST  = 1
	SOUTH = 2
	WEST  = 3
)

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func main() {
	s, _ := ioutil.ReadAll(os.Stdin)
	re := regexp.MustCompile(`[RL]\d+`)

	var x, y int
	var dir uint

	for _, step := range re.FindAllString(string(s), -1) {
		distance, _ := strconv.Atoi(step[1:])
		if step[0] == 'R' {
			dir = (dir + 1) % 4
		} else {
			dir = (dir - 1) % 4
		}

		switch dir {
		case NORTH:
			y += distance
		case EAST:
			x += distance
		case SOUTH:
			y -= distance
		case WEST:
			x -= distance
		}
	}

	fmt.Println("Part 1:", abs(x)+abs(y))
}
