package day01

import (
	"strconv"
	"strings"
)

type Day01 struct {
	elves [][]int
}

// Solve part 1
func (d *Day01) Part1() {

}

// Solve part 2
func (d *Day01) Part2() {

}

// Parse input into usable format
func (d *Day01) ParseInput(input string) {
	s := strings.Split(input, "\n\n")
	for _, elf := range s {
		snacks := strings.Split(elf, "\n")
		var row []int
		for _, snack := range snacks {
			calories, err := strconv.Atoi(snack)
			if err != nil {
				panic("Failed to convert string to int in input!")
			}
			row = append(row, calories)
		}
		d.elves = append(d.elves, row)
	}
}
