package aoc

import (
	"bufio"
	"io"
	"iter"
	"log"
	"strconv"
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

func Map[T1, T2 any](s []T1, f func(T1) T2) []T2 {
	var res []T2
	for _, v := range s {
		res = append(res, f(v))
	}
	return res
}

// Count how many elements in s satisfy f
func Count[T1 any](s []T1, f func(T1) bool) int {
	count := 0
	for _, v := range s {
		if f(v) {
			count++
		}
	}
	return count
}

// Atoi but panics on error
func Atoi(s string) int {
	val, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return val
}
