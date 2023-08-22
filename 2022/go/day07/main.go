package day07

import (
	"fmt"
	"strings"
)

type fileType int64

const (
	file fileType = iota
	directory
)

type node struct {
	children map[string]*node
	name     string
	size     int
	kind     fileType
	parent   *node
}

func (n *node) calcTotalSize() int {
	if n.kind == file {
		return n.size
	} else {
		sum := 0
		for _, node := range n.children {
			sum += node.calcTotalSize()
		}
		n.size = sum
		return sum
	}
}

type Day07 struct {
	filesystem *node
}

func (d *Day07) ParseInput(input string) {
	d.filesystem = &node{
		children: make(map[string]*node),
		name:     "/",
		size:     0,
		kind:     directory,
		parent:   nil,
	}

	currentNode := d.filesystem
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		//parse command
		if line[0] == '$' {
			if line[2:4] == "cd" {
				newDir := line[5:]
				if newDir == ".." && currentNode.parent != nil {
					currentNode = currentNode.parent
				} else if newDir == "/" {
					currentNode = d.filesystem
				} else {
					newNode, exists := currentNode.children[newDir]
					if !exists {
						newNode = &node{
							children: make(map[string]*node),
							name:     newDir,
							size:     0,
							kind:     directory,
							parent:   currentNode,
						}
						currentNode.children[newDir] = newNode
					}
					currentNode = newNode
				}
				// ignore ls command
			} else if line[2:4] == "ls" {
				continue
			} else {
				panic(fmt.Sprintf("Unrecognised command: %s", line))
			}
			// parse file information
		} else {
			// directory information
			if line[0:3] == "dir" {
				newDir := line[4:]
				newNode, exists := currentNode.children[newDir]
				if !exists {
					newNode = &node{
						children: make(map[string]*node),
						name:     newDir,
						size:     0,
						kind:     directory,
						parent:   currentNode,
					}
					currentNode.children[newDir] = newNode
				}
			} else {
				var size int
				var filename string
				nScanned, err := fmt.Sscanf(line, "%d %s", &size, &filename)
				if err != nil || nScanned != 2 {
					panic(fmt.Sprintf("Failed to parse file information: %s", line))
				}
				currentNode.children[filename] = &node{
					children: nil,
					name:     filename,
					size:     size,
					kind:     file,
					parent:   currentNode,
				}
			}
		}
	}

	d.filesystem.calcTotalSize()

}

func calcSumSizeWithMaxSize(n *node, maxSize int) int {
	res := 0
	if n.size <= maxSize {
		res += n.size
	}
	for _, child := range n.children {
		if child.kind == directory {
			res += calcSumSizeWithMaxSize(child, maxSize)
		}
	}
	return res
}

func (d *Day07) Part1() {
	maxSize := 100000
	result := calcSumSizeWithMaxSize(d.filesystem, maxSize)
	fmt.Println(result)

}

func findSmallestDirAboveSize(n *node, minSize int, currentSmallest *node) *node {
	newSmallest := currentSmallest
	if n.kind == directory && n.size < newSmallest.size && n.size >= minSize {
		newSmallest = n
	}
	for _, child := range n.children {
		smallestChild := findSmallestDirAboveSize(child, minSize, newSmallest)
		if smallestChild.size < newSmallest.size && smallestChild.size >= minSize && smallestChild.kind == directory {
			newSmallest = smallestChild
		}
	}
	return newSmallest
}

func (d *Day07) Part2() {
	totalSpace := 70000000
	updateSpace := 30000000
	availableSpace := totalSpace - d.filesystem.size
	neededSpace := updateSpace - availableSpace
	fmt.Println("Space need to be freed up for the update", neededSpace)

	smallest := findSmallestDirAboveSize(d.filesystem, neededSpace, d.filesystem)
	fmt.Println("Smallest node available for deletion is", smallest.name, "with size", smallest.size)

}
