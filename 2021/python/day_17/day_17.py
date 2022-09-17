#!/usr/bin/env python3

minx = 88; maxx = 125; miny = -157; maxy=-103

# sample
minx = 20; maxx = 30; miny=-10; maxy = -5


def main():
    test_x = [_ for _ in range(maxx+1)]
    possible_x = {}
    for x in test_x:
        p = 0
        n_steps = 0
        for drag in range(x):
            n_steps += 1
            p += x - drag
            if p <= maxx and p >= minx:
                possible_x[x] = n_steps
            if p > maxx:
                break
    print(possible_x)

    for y in range(miny, maxy + 1):
        print(y)


    # For part 1 we search the highest y that will work with the smallest x
    # how many steps does it take for smalles x to get inside area?



    # possible_y = [y for y in range(maxy // 2 , , 1)]
    # print(possible_y)



def main2():
    pass

if __name__ == '__main__':
    main()
    main2()