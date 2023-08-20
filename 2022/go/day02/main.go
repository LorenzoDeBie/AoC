package day02

import (
	"fmt"
	"strings"
)

type hand string

const (
	Rock     hand = "R"
	Paper         = "P"
	Scissors      = "S"
)

type moveResult int

const (
	Defeat moveResult = iota
	Draw
	Win
)

var (
	figureCodes = map[string]hand{
		"A": Rock,
		"B": Paper,
		"C": Scissors,
		"X": Rock,
		"Y": Paper,
		"Z": Scissors,
	}

	resultCodes = map[string]moveResult{
		"X": Defeat,
		"Y": Draw,
		"Z": Win,
	}

	figurePoints = map[hand]int{
		Rock:     1,
		Paper:    2,
		Scissors: 3,
	}

	rules = map[hand][2]hand{
		Rock:     {Paper, Scissors},
		Paper:    {Scissors, Rock},
		Scissors: {Rock, Paper},
	}
)

type move struct {
	otherHand hand
	col2      string
}

type Day02 struct {
	moves []move
}

func (d *Day02) ParseInput(input string) {
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		cols := strings.Split(line, " ")
		d.moves = append(d.moves, move{figureCodes[cols[0]], cols[1]})
	}
}

func (d *Day02) Part1() {
	score := 0
	for _, move := range d.moves {
		otherHand := move.otherHand
		myHand := figureCodes[move.col2]
		// always add score for my hand
		score += figurePoints[myHand]

		if otherHand == myHand {
			// draw
			score += 3
		} else if (myHand == Rock && otherHand == Scissors) ||
			(myHand == Paper && otherHand == Rock) ||
			(myHand == Scissors && otherHand == Paper) {
			// win
			score += 6

		}
	}
	fmt.Println(score)
}

func (d *Day02) Part2() {
	score := 0
	for _, move := range d.moves {
		otherHand := move.otherHand
		currentMoveResult := resultCodes[move.col2]
		var myHand hand
		switch currentMoveResult {
		case Win:
			myHand = rules[otherHand][0]
			score += 6
		case Draw:
			myHand = otherHand
			score += 3
		case Defeat:
			myHand = rules[otherHand][1]
		}
		score += figurePoints[myHand]
	}

	fmt.Println(score)

}
