package main

import (
	"fmt"
	"os"
	"strings"

	"czak.pl/aoc2024/aoc"
)

const (
	ex1 = `2333133121414131402`
	ex2 = `12345`
)

const (
	File int = iota
	Blank
)

type chunk struct {
	pos int
	len int
	id  int
}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	files := make([]chunk, 0, len(input))
	blanks := make([]chunk, 0, len(input))

	pos := 0
	id := 0
	t := File

	for _, r := range input {
		width := int(r - '0')

		if t == File {
			files = append(files, chunk{pos, width, id})
			id++
		} else {
			blanks = append(blanks, chunk{pos, width, -1})
		}

		pos += width
		t = 1 - t
	}

	for src := len(files) - 1; src > 0; src-- {
		// find first blank big enough to fit file
		for dst := range blanks {
			if blanks[dst].pos > files[src].pos {
				break
			}

			if blanks[dst].len >= files[src].len {
				// move file to blank
				files[src].pos = blanks[dst].pos

				// shrink blank
				blanks[dst].len -= files[src].len
				blanks[dst].pos += files[src].len

				break
			}
		}
	}

	total := 0

	for _, f := range files {
		for i := range f.len {
			total += f.id * (f.pos + i)
		}
	}

	fmt.Println(total)
}
