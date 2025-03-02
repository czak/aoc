package aoc

import (
	"bufio"
	"io"
	"iter"
	"log"
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

// Read from io.Reader until EOF into a string
func ReadAll(r io.Reader) string {
	b, err := io.ReadAll(r)
	if err != nil {
		log.Fatal(err)
	}
	return string(b)
}
