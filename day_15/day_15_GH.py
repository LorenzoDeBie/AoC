# chiton.py

import heapq
import os
import sys
from functools import total_ordering

import numpy as np


@total_ordering
class Node:
    def __init__(self, coor: complex, value: int, parent):
        self.value = value
        self.coor = coor
        self.parent = parent
        self.h = 0
        self.g = 0
        self.f = 0

    def __repr__(self):
        return repr((self.coor, self.value))

    def __eq__(self, other):
        if isinstance(other, Node):
            return self.coor == other.coor
        elif isinstance(other, complex):
            return self.coor == other
        else:
            return False

    def __lt__(self, other):
        if isinstance(other, Node):
            return (self.coor.real < other.coor.real) and (self.coor.imag < other.coor.imag)
        elif isinstance(other, complex):
            return (self.coor.real < other.real) and (self.coor.imag < other.imag)
        else:
            return False

    def __hash__(self):
        return hash(self.coor)

    def add(self, other, value):
        return Node(complex(self.coor.real + other.real, self.coor.imag + other.imag), value, self)


def read_and_init(fn):
    with open(fn, 'r') as fo:
        lines = fo.read().splitlines()
    cave = {}
    for y in range(len(lines)):
        for x in range(len(lines[0])):
            cave[x + 1j * y] = int(lines[y][x])
    return cave


def np_cycle_inc(arr: np.ndarray, addend: int):
    new_arr = np.copy(arr)
    for _ in range(addend):
        new_arr += 1
        new_arr = np.where(new_arr > 9, 1, new_arr)
    return new_arr


def map_expander(cave) -> dict:
    changes = [[0, 1, 2, 3, 4],
               [1, 2, 3, 4, 5],
               [2, 3, 4, 5, 6],
               [3, 4, 5, 6, 7],
               [4, 5, 6, 7, 8]]
    changes = np.array(changes, dtype=np.int32)
    cell = np.zeros((int(max(k.imag for k in cave)) + 1, int(max(k.real for k in cave)) + 1), dtype=np.int32)
    for y in range(int(max(k.imag for k in cave)) + 1):
        for x in range(int(max(k.real for k in cave)) + 1):
            cell[y, x] = cave[x + 1j * y]
    new_cave = None
    for row in range(5):
        new_row = np_cycle_inc(cell, changes[row, 0])
        for col in range(1, 5):
            new_cell = np_cycle_inc(cell, changes[row, col])
            new_row = np.concatenate((new_row, new_cell), axis=1)
        if row == 0:
            new_cave = new_row
        else:
            new_cave = np.concatenate((new_cave, new_row), axis=0)
    return {col+1j*row: new_cave[row, col] for row in range(new_cave.shape[0]) for col in range(new_cave.shape[1])}


def in_bounds(cave, location: complex, x_max: int, y_max: int) -> bool:
    return (0 <= location.real < x_max) and (0 <= location.imag < y_max)


def display(cave):
    buffer = []
    for y in range(int(max(k.imag for k in cave)) + 1):
        line = ''
        for x in range(int(max(k.real for k in cave)) + 1):
            line += str(cave[x+1j*y])
        buffer.append(line)
    for line in buffer:
        print(line)


def dijkstra(cave: dict, start: complex, end: complex):
    distance = {k: sys.maxsize for k in cave}
    distance[start] = 0
    previous = {k: None for k in cave}
    visited = set()
    queue = []
    vecs = (0 - 1j, -1 + 0j, 1 + 0j, 0 + 1j)
    x_max, y_max = int(max(k.real for k in cave)) + 1, int(max(k.imag for k in cave)) + 1
    start_node = Node(start, cave[start], None)
    heapq.heappush(queue, (distance[start], start_node))
    while queue:
        _, current = heapq.heappop(queue)
        visited.add(current)
        if current.coor == end:
            break
        neighbors = [current.add(vec, cave[current.coor + vec]) for vec in vecs if in_bounds(cave,
                                                                                             current.coor + vec,
                                                                                             x_max,
                                                                                             y_max)]
        for neighbor in neighbors:
            if neighbor not in visited:
                neighbor.f = current.f + cave[neighbor.coor]
                if neighbor.f < distance[neighbor.coor]:
                    distance[neighbor.coor] = neighbor.f
                    previous[neighbor.coor] = current.coor
                    heapq.heappush(queue, (distance[neighbor.coor], neighbor))
    return distance[end]


def part1(cave):
    start = 0j
    end = complex(max(k.real for k in cave), max(k.imag for k in cave))
    min_dist = dijkstra(cave, start, end)
    print('Part 1:', min_dist)


def part2(cave):
    cave = map_expander(cave)
    start = 0j
    end = complex(max(k.real for k in cave), max(k.imag for k in cave))
    min_dist = dijkstra(cave, start, end)
    print('Part 2:', min_dist)


c = read_and_init(os.path.dirname(__file__) + '/day_15.txt')
part1(c)
part2(c)