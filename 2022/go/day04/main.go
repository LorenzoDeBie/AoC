package day04

import (
	"strconv"
	"strings"
)

type elf struct {
	start int
	end   int
}

type Day04 struct {
	pairs [][2]elf
}

func (d *Day04) ParseInput(input string) {
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		elves := strings.Split(line, ",")
		elf1 := strings.Split(elves[0], "-")
		elf1Begin, _ := strconv.Atoi(elf1[0])
		elf1End, _ := strconv.Atoi(elf1[1])
		elf2 := strings.Split(elves[1], "-")
		elf2Begin, _ := strconv.Atoi(elf2[0])
		elf2End, _ := strconv.Atoi(elf2[1])
		d.pairs = append(d.pairs, [2]elf{
			{elf1Begin, elf1End},
			{elf2Begin, elf2End},
		})
	}
}

func (d *Day04) Part1() {
	count := 0
	for _, pair := range d.pairs {
		if pair[0].start <= pair[1].start && pair[0].end >= pair[1].end {
			count++
		} else if pair[1].start <= pair[0].start && pair[1].end >= pair[0].end {
			count++
		}
	}
	println(count)
}

func (d *Day04) Part2() {
	count := 0
	for _, pair := range d.pairs {
		// fully enclose
		if pair[0].start <= pair[1].start && pair[0].end >= pair[1].end ||
			pair[1].start <= pair[0].start && pair[1].end >= pair[0].end ||
			// part overlap
			pair[0].start <= pair[1].start && pair[0].end >= pair[1].start ||
			pair[1].start <= pair[0].start && pair[1].end >= pair[0].start {
			count++
		}
	}
	println(count)
}
