#!/usr/bin/env python3

from collections import defaultdict

input = '3,5,1,5,3,2,1,3,4,2,5,1,3,3,2,5,1,3,1,5,5,1,1,1,2,4,1,4,5,2,1,2,4,3,1,2,3,4,3,4,4,5,1,1,1,1,5,5,3,4,4,4,5,3,4,1,4,3,3,2,1,1,3,3,3,2,1,3,5,2,3,4,2,5,4,5,4,4,2,2,3,3,3,3,5,4,2,3,1,2,1,1,2,2,5,1,1,4,1,5,3,2,1,4,1,5,1,4,5,2,1,1,1,4,5,4,2,4,5,4,2,4,4,1,1,2,2,1,1,2,3,3,2,5,2,1,1,2,1,1,1,3,2,3,1,5,4,5,3,3,2,1,1,1,3,5,1,1,4,4,5,4,3,3,3,3,2,4,5,2,1,1,1,4,2,4,2,2,5,5,5,4,1,1,5,1,5,2,1,3,3,2,5,2,1,2,4,3,3,1,5,4,1,1,1,4,2,5,5,4,4,3,4,3,1,5,5,2,5,4,2,3,4,1,1,4,4,3,4,1,3,4,1,1,4,3,2,2,5,3,1,4,4,4,1,3,4,3,1,5,3,3,5,5,4,4,1,2,4,2,2,3,1,1,4,5,3,1,1,1,1,3,5,4,1,1,2,1,1,2,1,2,3,1,1,3,2,2,5,5,1,5,5,1,4,4,3,5,4,4'

#input = '3,4,3,1,2'

days = 256

def main():
    lanterfishes = [int(x) for x in input.split(',')]
    for day in range(days):
        for i in range(len(lanterfishes)):
            lanterfishes[i] -= 1
            if lanterfishes[i] < 0:
                lanterfishes.append(8)
                lanterfishes[i] = 6
    print(lanterfishes)
    print(len(lanterfishes))

def main2():
    lanterfishes = { x: 0 for x in range(-1, 9, 1)}
    for lanterfish in [int(x) for x in input.split(',')]:
        lanterfishes[lanterfish] += 1

    print(lanterfishes)
    
    for day in range(days):
        inumber = lanterfishes[8]
        lanterfishes[8] = 0
        for i in range(7, -2, -1):
            nexti = lanterfishes[i]
            lanterfishes[i] = inumber
            inumber = nexti
            if i == -1:
                lanterfishes[6] += lanterfishes[-1]
                lanterfishes[8] += lanterfishes[-1]
        print(lanterfishes)
        lanterfishes[-1] = 0
        print(sum(lanterfishes.values()))
if __name__ == '__main__':
    main2()