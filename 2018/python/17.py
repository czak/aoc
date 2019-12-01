from parse import parse
import pdb
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

M = [
    list('..............'),
    list('............#.'),
    list('.#........#.#.'),
    list('.#..###...#...'),
    list('.#........#...'),
    list('.#........#...'),
    list('.#........#...'),
    list('.####..####...'),
    list('..............'),
    list('..............'),
    list('....#.....#...'),
    list('....#.....#...'),
    list('....#.....#...'),
    list('....#######...'),
    ]

def printmap(M, filled, f):
    for y, row in enumerate(M):
        for x, cell in enumerate(row):
            if (x, y) in filled:
                if (x, y) in cache and ground in cache[(x, y)] or (x, y) in lcache and ground in lcache[(x, y)] or (x, y) in rcache and ground in rcache[(x, y)]:
                    print('|', end='', file=f)
                else:
                    print('~', end='', file=f)
            else:
                print(cell, end='', file=f)
        print('', file=f)

ground = 'G'

cache = {}
lcache = {}
rcache = {}

def fill(M, x, y):
    def fillleft(M, x, y):
        if (x, y) in lcache:
            return lcache[(x, y)]
        if M[y][x] == '#':
            res = set()
            lcache[(x, y)] = res
            return res
        below = fill(M, x, y+1)
        if ground in below:
            res = set([(x, y)]) | below
            lcache[(x, y)] = res
            return res
        res = set([(x, y)]) | below | fillleft(M, x-1, y) | fillright(M, x+1, y)
        lcache[(x, y)] = res
        return res

    def fillright(M, x, y):
        if (x, y) in rcache:
            return rcache[(x, y)]
        if x == len(M[y]) or M[y][x] == '#':
            res = set()
            rcache[(x, y)] = res
            return res
        below = fill(M, x, y+1)
        if ground in below:
            res = set([(x, y)]) | below
            rcache[(x, y)] = res
            return res
        res = set([(x, y)]) | below | fillright(M, x+1, y)
        rcache[(x, y)] = res
        return res

    if (x, y) in cache:
        print('cached', x, y)
        return cache[(x, y)]
    if y == len(M):
        res = set(ground)
        cache[(x, y)] = res
        return res
    if x < 0 or x > len(M[y])-1 or M[y][x] == '#':
        res = set()
        cache[(x, y)] = res
        return res
    below = fill(M, x, y+1)
    if ground in below:
        res = set([(x, y)]) | below
        cache[(x, y)] = res
        return res
    res = set([(x, y)]) | below | fillleft(M, x-1, y) | fillright(M, x+1, y)
    cache[(x, y)] = res
    return res

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

print(xlo, xhi)
print(ylo, yhi)

M = [['.' for x in range(xhi+1)] for y in range(yhi+1)]

for y in range(yhi+1):
    for x in range(xhi+1):
        if (x, y) in clay:
            M[y][x] = '#'

sys.setrecursionlimit(10000)

filled = fill(M, 500, 0)

with open('17-2.out', 'w') as f:
    printmap(M, filled, f)

# filled = fill(M, 5, 0)
# printmap(M, filled, sys.stdout)

counted = set(p for p in filled if p is not ground and p[1] >= ylo and p[1] <= yhi)
print('counted', len(counted))
