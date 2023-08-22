package main

import (
	"flag"
	"fmt"
	"github.com/LorenzoDeBie/AoC/2022/go/day01"
	"github.com/LorenzoDeBie/AoC/2022/go/day02"
	"github.com/LorenzoDeBie/AoC/2022/go/day03"
	"github.com/LorenzoDeBie/AoC/2022/go/day04"
	"github.com/LorenzoDeBie/AoC/2022/go/day05"
	"github.com/LorenzoDeBie/AoC/2022/go/globals"
	"os"
)

type aoc2022Challenge interface {
	ParseInput(input string)
	Part1()
	Part2()
}

func solveDay(day int, challenge aoc2022Challenge) {
	fmt.Println("Solving Challenge", day+1)
	fmt.Println("Reading input file")

	inputFile := fmt.Sprintf("day%02d/input.txt", day+1)
	if globals.UseExampleInput {
		inputFile = fmt.Sprintf("day%02d/input_test.txt", day+1)
	}
	data, err := os.ReadFile(inputFile)
	if err != nil {
		panic("Failed to read input data")
	}

	fmt.Println("Read input file:", inputFile)
	fmt.Println("Parsing input data")
	challenge.ParseInput(string(data))
	fmt.Println("Solving Part 1")
	challenge.Part1()
	fmt.Println("Solving Part 2")
	challenge.Part2()
	fmt.Println()
}

func main() {
	fmt.Println(globals.UseExampleInput)
	var day int
	flag.BoolVar(&globals.UseExampleInput, "example", false, "Use the example input instead of the real input")
	flag.IntVar(&day, "day", 0, "Which day to solve, 0 for all days")
	flag.Parse()
	fmt.Println(globals.UseExampleInput)

	challenges := []aoc2022Challenge{
		&day01.Day01{},
		&day02.Day02{},
		&day03.Day03{},
		&day04.Day04{},
		day05.New(),
	}

	if day == 0 {
		fmt.Println("Solving all challenges")
		for day, challenge := range challenges {
			solveDay(day, challenge)
		}
	} else {
		solveDay(day-1, challenges[day-1])
	}

}
