package aoc

import (
	"bufio"
	"io"
	"iter"
)

// Iterate over lines of reader
func Lines(reader io.Reader) iter.Seq[string] {
	return func(yield func(string) bool) {
		scanner := bufio.NewScanner(reader)
		for scanner.Scan() {
			if !yield(scanner.Text()) {
				return
			}
		}
	}
}
