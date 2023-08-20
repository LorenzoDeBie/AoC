package day03

import (
	"fmt"
	"github.com/emirpasic/gods/sets/hashset"
	"strings"
)

type backpack struct {
	compartment1 *hashset.Set
	compartment2 *hashset.Set
}

func (b backpack) findCommonLetter() rune {
	intersection := b.compartment1.Intersection(b.compartment2)
	if intersection.Size() > 1 {
		panic("More than one common letter in compartments")
	} else if intersection.Size() == 0 {
		panic("No common letter in compartments has been found!")
	}
	value := rune(intersection.Values()[0].(string)[0])
	return value
}

func (b backpack) getFullSet() *hashset.Set {
	result := hashset.New(b.compartment1.Values()...)
	result.Add(b.compartment2.Values()...)
	return result
}

func getPriorityOfLetter(r rune) int {
	if r >= 'a' {
		return int(r-rune('a')) + 1
	} else {
		return int(r-rune('A')) + 27
	}
}

type Day03 struct {
	backpacks []backpack
}

func (d *Day03) ParseInput(input string) {
	lines := strings.Split(input, "\n")
	for i, line := range lines {
		d.backpacks = append(d.backpacks, backpack{
			compartment1: hashset.New(),
			compartment2: hashset.New(),
		})
		lineLen := len(line)
		compartment1String := line[:(lineLen / 2)]
		compartment2String := line[lineLen/2:]

		compartment1Letters := strings.Split(compartment1String, "")
		compartment2Letters := strings.Split(compartment2String, "")

		for _, letter := range compartment1Letters {
			d.backpacks[i].compartment1.Add(letter)
		}
		for _, letter := range compartment2Letters {
			d.backpacks[i].compartment2.Add(letter)
		}

	}
}

func (d *Day03) Part1() {
	result := 0
	for _, b := range d.backpacks {
		result += getPriorityOfLetter(b.findCommonLetter())
	}
	fmt.Println(result)
}

func (d *Day03) Part2() {
	result := 0
	for i := 0; i < len(d.backpacks); i += 3 {
		backpack1 := d.backpacks[i].getFullSet()
		backpack2 := d.backpacks[i+1].getFullSet()
		backpack3 := d.backpacks[i+2].getFullSet()

		intersection := backpack1.Intersection(backpack2).Intersection(backpack3)
		result += getPriorityOfLetter(rune(intersection.Values()[0].(string)[0]))
	}
	fmt.Println(result)
}
