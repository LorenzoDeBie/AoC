package day05

import (
	"fmt"
	"github.com/LorenzoDeBie/AoC/2022/go/globals"
	"github.com/emirpasic/gods/stacks/arraystack"
	"strings"
)

var numberOfStacks int
var inputCopy string

type craneOperation struct {
	quantity    int
	source      int
	destination int
}

type Day05 struct {
	stacks          []*arraystack.Stack
	craneOperations []craneOperation
}

func New() *Day05 {
	fmt.Println("in new", globals.UseExampleInput)
	if globals.UseExampleInput {
		numberOfStacks = 3
	} else {
		numberOfStacks = 9
	}
	res := Day05{stacks: make([]*arraystack.Stack, numberOfStacks)}
	for i := 0; i < len(res.stacks); i++ {
		res.stacks[i] = arraystack.New()
	}
	return &res
}

func (d *Day05) printStacks() {
	for i, stack := range d.stacks {
		peek, ok := stack.Peek()
		if ok {
			fmt.Print(i+1, ", ", string(peek.(rune)), ": ")
		} else {
			fmt.Print(i+1, ": ")
		}
		for _, value := range stack.Values() {
			fmt.Print(string(value.(rune)), " ")
		}
		fmt.Println()
	}
}

func (d *Day05) ParseInput(input string) {
	inputCopy = input
	for _, stack := range d.stacks {
		stack.Clear()
	}
	stacks := make([]*arraystack.Stack, numberOfStacks)
	for i := 0; i < len(stacks); i++ {
		stacks[i] = arraystack.New()
	}

	lines := strings.Split(input, "\n")
	i := 0
	line := lines[i]
	for line[1] != '1' {
		for stack := 0; stack < numberOfStacks; stack++ {
			index := 1 + stack*4
			if index > len(line) {
				continue
			}
			crate := rune(line[index])
			if crate != ' ' {
				stacks[stack].Push(crate)
			}
		}
		i++
		line = lines[i]
	}
	for stacki, stack := range stacks {
		val, ok := stack.Pop()
		for ok {
			d.stacks[stacki].Push(val)
			val, ok = stack.Pop()
		}
	}
	d.printStacks()
	i += 2
	d.craneOperations = nil
	d.craneOperations = []craneOperation{}
	for _, s := range lines[i:] {
		var quantity, source, destination int
		_, err := fmt.Sscanf(s, "move %d from %d to %d", &quantity, &source, &destination)
		if err != nil {
			panic("Failed to parse crane operation")
		}
		d.craneOperations = append(d.craneOperations, craneOperation{quantity, source, destination})
	}
}

func (d *Day05) Part1() {
	d.printStacks()
	for _, operation := range d.craneOperations {
		for i := 0; i < operation.quantity; i++ {
			crate, ok := d.stacks[operation.source-1].Pop()
			if ok {
				d.stacks[operation.destination-1].Push(crate)
			}
		}
		d.printStacks()
	}
	for _, stack := range d.stacks {
		msg, ok := stack.Peek()
		if ok {
			fmt.Print(string(msg.(rune)))
		}
	}
	fmt.Println()
}

func (d *Day05) Part2() {
	d.ParseInput(inputCopy)
	d.printStacks()
	for _, operation := range d.craneOperations {
		holdingStack := arraystack.New()
		for i := 0; i < operation.quantity; i++ {
			crate, ok := d.stacks[operation.source-1].Pop()
			if ok {
				holdingStack.Push(crate)
			}
		}
		crate, ok := holdingStack.Pop()
		for ok {
			d.stacks[operation.destination-1].Push(crate)
			crate, ok = holdingStack.Pop()
		}
		d.printStacks()
	}
	for _, stack := range d.stacks {
		msg, ok := stack.Peek()
		if ok {
			fmt.Print(string(msg.(rune)))
		}
	}
	fmt.Println()
}
