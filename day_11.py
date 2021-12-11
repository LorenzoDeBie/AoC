#!/usr/bin/env python3

input = '''7232374314
8531113786
3411787828
5482241344
5856827742
7614532764
5311321758
1255116187
5821277714
2623834788'''.splitlines()


# input = '''5483143223
# 2745854711
# 5264556173
# 6141336146
# 6357385478
# 4167524645
# 2176841721
# 6882881134
# 4846848554
# 5283751526'''.splitlines()

from pprint import pprint

steps = 100

def main():
    # parse numbers
    numbers = [[int(x) for x in line] for line in input]
    # add border around the numbers
    for line in numbers:
        line.insert(0, -999)
        line.append(-999)
    numbers.insert(0, [-999 for _ in range(len(numbers[0]))])
    numbers.append([-999 for _ in range(len(numbers[0]))])
    pprint(numbers)

    n_flashes = 0
    # 100 steps
    for i in range(steps):
        flashed = [[False for x in row] for row in numbers]
        # increase all numbers by 1
        numbers = [[x + 1 for x in row] for row in numbers]
        # check which octopuses flash
        print(any(any(numbers[row][col] > 9 and not flashed[row][col] for col in range(1, len(numbers[row]) - 1)) for row in range(1, len(numbers) - 1)))
        while any(any(numbers[row][col] > 9 and not flashed[row][col] for col in range(1, len(numbers[row]) - 1)) for row in range(1, len(numbers) - 1)):
            for row in range(1, len(numbers) - 1):
                for col in range(1, len(numbers[row]) - 1):
                    n = numbers[row][col]
                    if n > 9 and not flashed[row][col]:
                        flashed[row][col] = True
                        n_flashes += 1
                        # increase all neighbours
                        numbers[row + 1][col] += 1
                        numbers[row - 1][col] += 1
                        numbers[row][col + 1] += 1
                        numbers[row][col - 1] += 1
                        numbers[row + 1][col + 1] += 1
                        numbers[row + 1][col - 1] += 1
                        numbers[row - 1][col + 1] += 1
                        numbers[row - 1][col - 1] += 1
        numbers = [[x if x <= 9 else 0 for x in row] for row in numbers]
        pprint(numbers)
        pprint(flashed, width=135)
    print(n_flashes)


                    

def main2():
    # parse numbers
    numbers = [[int(x) for x in line] for line in input]
    # add border around the numbers
    for line in numbers:
        line.insert(0, -999)
        line.append(-999)
    numbers.insert(0, [-999 for _ in range(len(numbers[0]))])
    numbers.append([-999 for _ in range(len(numbers[0]))])
    pprint(numbers)

    n_flashes = 0
    i = 0
    while True:
        flashed = [[False for x in row] for row in numbers]
        # increase all numbers by 1
        numbers = [[x + 1 for x in row] for row in numbers]
        # check which octopuses flash
        #print(any(any(numbers[row][col] > 9 and not flashed[row][col] for col in range(1, len(numbers[row]) - 1)) for row in range(1, len(numbers) - 1)))
        while any(any(numbers[row][col] > 9 and not flashed[row][col] for col in range(1, len(numbers[row]) - 1)) for row in range(1, len(numbers) - 1)):
            for row in range(1, len(numbers) - 1):
                for col in range(1, len(numbers[row]) - 1):
                    n = numbers[row][col]
                    if n > 9 and not flashed[row][col]:
                        flashed[row][col] = True
                        n_flashes += 1
                        # increase all neighbours
                        numbers[row + 1][col] += 1
                        numbers[row - 1][col] += 1
                        numbers[row][col + 1] += 1
                        numbers[row][col - 1] += 1
                        numbers[row + 1][col + 1] += 1
                        numbers[row + 1][col - 1] += 1
                        numbers[row - 1][col + 1] += 1
                        numbers[row - 1][col - 1] += 1
        if all(all(flashed[row][col] for col in range(1, len(numbers[row]) - 1)) for row in range(1, len(numbers) - 1)):
            print('ALL ARE LIGHTING UP', i + 1)
            pprint(flashed, width=135)
            break
        numbers = [[x if x <= 9 else 0 for x in row] for row in numbers]
        # pprint(numbers)
        # pprint(flashed, width=135)
        i += 1
        print(i)
    print(n_flashes)

if __name__ == '__main__':
    main()
    main2()