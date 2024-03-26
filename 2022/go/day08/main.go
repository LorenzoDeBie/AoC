package day08

import (
	"fmt"
	"github.com/emirpasic/gods/sets/hashset"
	"slices"
	"strconv"
	"strings"
)

type Day08 struct {
	// use 1d array as 2d: idx = y * cols + x
	grid       []int
	cols, rows int
}

func (d *Day08) ParseInput(input string) {
	lines := strings.Split(input, "\n")
	d.rows = len(lines)
	d.cols = len(lines[0])
	d.grid = make([]int, d.cols*d.rows)

	for row, line := range lines {
		numbers := strings.Split(line, "")
		for col, number := range numbers {
			n, err := strconv.Atoi(number)
			if err != nil {
				panic(fmt.Sprintf("Failed to convert %s to int!", number))
			}
			d.grid[row*d.cols+col] = n
		}
	}
}

func (d *Day08) Part1() {
	visibleTreesIdxs := hashset.New()
	// scan all cols top to bottom
	for col := 1; col < d.cols-1; col++ {
		max := d.grid[col]
		for row := 1; row < d.rows-1; row++ {
			idx := row*d.cols + col
			value := d.grid[idx]
			if value > max {
				max = value
				visibleTreesIdxs.Add(idx)
			}
		}
	}
	// scan all cols bottom to top
	for col := 1; col < d.cols-1; col++ {
		max := d.grid[(d.rows-1)*d.rows+col]
		for row := d.rows - 2; row > 0; row-- {
			idx := row*d.cols + col
			value := d.grid[idx]
			if value > max {
				max = value
				visibleTreesIdxs.Add(idx)
			}
		}
	}
	//scan all rows left to right
	for row := 1; row < d.rows-1; row++ {
		max := d.grid[row*d.cols]
		for col := 1; col < d.cols-1; col++ {
			idx := row*d.cols + col
			value := d.grid[idx]
			if value > max {
				max = value
				visibleTreesIdxs.Add(idx)
			}
		}
	}
	// scan all rows right to left
	for row := 1; row < d.rows-1; row++ {
		max := d.grid[(row+1)*d.cols-1]
		for col := d.cols - 2; col > 0; col-- {
			idx := row*d.cols + col
			value := d.grid[idx]
			if value > max {
				max = value
				visibleTreesIdxs.Add(idx)
			}
		}
	}

	totalVisible := visibleTreesIdxs.Size() + (2 * d.cols) + 2*(d.rows-2)
	fmt.Println(totalVisible, "trees are visible")
}

func (d *Day08) Part2() {
	scenicScores := make([]int, d.cols*d.rows)
	for i := range scenicScores {
		scenicScores[i] = 1
	}
	// loop over all values we are interested in (not the outer ring)
	for col := 1; col < d.cols-1; col++ {
		for row := 1; row < d.rows-1; row++ {
			idx := row*d.cols + col
			value := d.grid[idx]
			// look up
			score := 1
			counter := 1
			maxSeen := d.grid[idx-d.cols]
			for i := row - 1; i < 0; i-- {
				valueCompare := d.grid[i*d.cols+col]
				// we canot look post tree of our height or higher
				if valueCompare >= value {
					counter++
					break
				}
				if valueCompare > maxSeen {
					counter++
					maxSeen = valueCompare
				}
			}
			score *= counter
			// look down
			counter = 1
			maxSeen = d.grid[idx+d.cols]
			for i := row + 1; i < d.rows-1; i++ {
				valueCompare := d.grid[i*d.cols+col]
				// we canot look post tree of our height or higher
				if valueCompare >= value {
					counter++
					break
				}
				if valueCompare > maxSeen {
					counter++
					maxSeen = valueCompare
				}
			}
			score *= counter
			// look left
			counter = 1
			maxSeen = d.grid[idx-1]
			for i := col - 1; i > 0; i-- {
				valueCompare := d.grid[row*d.cols+i]
				// we canot look post tree of our height or higher
				if valueCompare >= value {
					counter++
					break
				}
				if valueCompare > maxSeen {
					counter++
					maxSeen = valueCompare
				}
			}
			score *= counter
			// look right
			counter = 1
			maxSeen = d.grid[idx+1]
			for i := col + 1; i < d.cols-1; i++ {
				valueCompare := d.grid[row*d.cols+i]
				// we canot look post tree of our height or higher
				if valueCompare >= value {
					counter++
					break
				}
				if valueCompare > maxSeen {
					counter++
					maxSeen = valueCompare
				}
			}
			score *= counter

			// save score in new slice
			scenicScores[idx] = score
		}
		fmt.Println("Max Scenic Score:", slices.Max(scenicScores))
	}

}
