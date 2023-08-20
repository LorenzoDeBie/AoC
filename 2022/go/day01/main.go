package day01

import (
	"sort"
	"strconv"
	"strings"
)

type Day01 struct {
	elves  [][]int
	totals []int
}

// Solve part 1
func (d *Day01) Part1() {
	result := 0
	for _, elf := range d.elves {
		sum := 0
		for _, snack := range elf {
			sum += snack
		}
		if sum > result {
			result = sum
		}
		d.totals = append(d.totals, sum)
	}
	println(result)
}

// Solve part 2
func (d *Day01) Part2() {
	sort.Ints(d.totals)
	sort.Sort(sort.Reverse(sort.IntSlice(d.totals)))
	println(d.totals[0] + d.totals[1] + d.totals[2])

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
