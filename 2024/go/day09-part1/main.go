package main

import (
	"fmt"
	"math"
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

type block struct {
	pos int
	id  int
}

func main() {
	// input := ex1
	input := strings.TrimSpace(aoc.ReadAll(os.Stdin))

	files := make([]block, 0, len(input)*10)
	blanks := make([]block, 0, len(input)*10)

	pos := 0
	id := 0
	t := File

	for _, r := range input {
		width := int(r - '0')

		if t == File {
			for range width {
				files = append(files, block{pos, id})
				pos++
			}
			id++
		} else {
			for range width {
				blanks = append(blanks, block{pos, 0})
				pos++
			}
		}

		t = 1 - t
	}

	moved := 0

	for dst := range blanks {
		src := len(files) - 1 - dst

		// all gaps filled?
		if blanks[dst].pos > files[src].pos {
			break
		}

		id := files[src].id
		blanks[dst].id = id

		moved++
	}

	files = files[:len(files)-moved]
	blanks = blanks[:moved]

	fi, bi := 0, 0
	total := 0

	for fi != len(files) || bi != len(blanks) {
		fpos := math.MaxInt
		if fi < len(files) {
			fpos = files[fi].pos
		}

		bpos := math.MaxInt
		if bi < len(blanks) {
			bpos = blanks[bi].pos
		}

		var blk block
		if fpos < bpos {
			blk = files[fi]
			fi++
		} else {
			blk = blanks[bi]
			bi++
		}

		total += blk.id * blk.pos
	}

	fmt.Println(total)
}
