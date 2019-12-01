# from parse import parse
# import pdb
import sys

M = [
    list('..............'),
    list('............#.'),
    list('.#..#.......#.'),
    list('.#..#..#......'),
    list('.#..#..#......'),
    list('.#.....#......'),
    list('.#.....#......'),
    list('.#######......'),
    list('..............'),
    list('..............'),
    list('....#.....#...'),
    list('....#.....#...'),
    list('....#.....#...'),
    list('....#######...'),
    ]

def printmap(M, filled, f=sys.stdout):
    for y, row in enumerate(M):
        for x, cell in enumerate(row):
            if (x, y) in filled:
                print('|', end='', file=f)
            else:
                print(cell, end='', file=f)
        print('', file=f)

ground = 'G'

def memoize(f):
    memo = {}
    def helper(M, x, y):
        if (x, y) not in memo:            
            memo[(x, y)] = f(M, x, y)
        return memo[(x, y)]
    return helper

@memoize
def filll(M, x, y):
    # print('l', x, y)

    if M[y][x] == '#':
        return set()

    below = fill(M, x, y+1)

    if ground in below:
        return {(x, y)} | below
    else:
        return {(x, y)} | below | filll(M, x-1, y)

@memoize
def fillr(M, x, y):
    # print('r', x, y)

    if M[y][x] == '#':
        return set()

    below = fill(M, x, y+1)

    if ground in below:
        return {(x, y)} | below
    else:
        return {(x, y)} | below | fillr(M, x+1, y)

visited = set()

def fill(M, x, y):
    # print('d', x, y)

    if y == len(M):
        return {ground}
    if M[y][x] == '#':
        return set()

    below = fill(M, x, y+1)

    if ground in below:
        return {(x, y)} | below
    else:
        left = filll(M, x-1, y)
        return {(x, y)} | below | filll(M, x-1, y) | fillr(M, x+1, y)

def load():
    clay = set()

    for line in open('17.in'):
        res = parse('x={:d}, y={:d}..{:d}', line)
        if res:
            x, y1, y2 = res
            clay |= set((x, y) for y in range(y1, y2+1))
        res = parse('y={:d}, x={:d}..{:d}', line)
        if res:
            y, x1, x2 = res
            clay |= set((x, y) for x in range(x1, x2+1))

    xlo, xhi = min(p[0] for p in clay), max(p[0] for p in clay)
    ylo, yhi = min(p[1] for p in clay), max(p[1] for p in clay)

    M = [['.' for x in range(xhi+1)] for y in range(yhi+1)]

    for y in range(yhi+1):
        for x in range(xhi+1):
            if (x, y) in clay:
                M[y][x] = '#'

    return M

# example
# filled = fill(M, 5, 0)
# printmap(M, filled, sys.stdout)

sys.setrecursionlimit(10000)

# main
import altin
filled = fill(altin.M, 500, 0)
print(len(filled))
