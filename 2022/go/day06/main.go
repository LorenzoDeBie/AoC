package day06

import (
	"fmt"
	"github.com/emirpasic/gods/sets/hashset"
	"strings"
)

type Day06 struct {
	buffers []string
}

func (d *Day06) ParseInput(input string) {
	d.buffers = strings.Split(input, "\n")
}

func (d *Day06) Part1() {
	for _, buffer := range d.buffers {
		for i := 3; i < len(buffer); i++ {
			packetSet := hashset.New()
			packetSet.Add(buffer[i-3], buffer[i-2], buffer[i-1], buffer[i])
			if packetSet.Size() == 4 {
				fmt.Println("Found start of packet at place", i+1)
				break
			}
		}
	}
}

func (d *Day06) Part2() {
	numberOfDistinctChars := 14
	for _, buffer := range d.buffers {
		for i := numberOfDistinctChars - 1; i < len(buffer); i++ {
			packetSet := hashset.New()
			for j := 0; j < numberOfDistinctChars; j++ {
				packetSet.Add(buffer[i-j])
			}
			if packetSet.Size() == numberOfDistinctChars {
				fmt.Println("Found start of packet at place", i+1)
				break
			}
		}
	}
}
