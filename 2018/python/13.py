from pprint import pprint
from itertools import cycle

# tracks = [
#     list('/->-\        '),
#     list('|   |  /----\\'),
#     list('| /-+--+-\  |'),
#     list('| | |  | v  |'),
#     list('\-+-/  \-+--/'),
#     list('  \------/   '),
# ]

tracks = [list(line) for line in open('13.in')]

BEND_MAP = {
        (0, -1): {
            '\\': 'left',
            '/': 'right',
            },
        (0, +1): {
            '\\': 'left',
            '/': 'right',
            },
        (-1, 0): {
            '\\': 'right',
            '/': 'left',
            },
        (+1, 0): {
            '\\': 'right',
            '/': 'left',
            },
        }

TURN_MAP = {
        (0, -1): {
            'left': (-1, 0),
            'right': (+1, 0),
            },
        (0, +1): {
            'left': (+1, 0),
            'right': (-1, 0),
            },
        (-1, 0): {
            'left': (0, +1),
            'right': (0, -1),
            },
        (+1, 0): {
            'left': (0, -1),
            'right': (0, +1),
            },
        }

class Cart():
    def __init__(self):
        self.crashed = False
        self.crossroad = cycle(['left', 'straight', 'right'])
    def __repr__(self):
        return f"({'#' if self.crashed else ''}{self.x},{self.y})=>({self.dx},{self.dy})"
    def turn(self, direction):
        if direction == 'straight':
            return
        self.dx, self.dy = TURN_MAP[(self.dx, self.dy)][direction]
    def bend(self, b):
        direction = BEND_MAP[(self.dx, self.dy)][b]
        self.turn(direction)
    def advance(self):
        self.x += self.dx
        self.y += self.dy



carts = []

directions = {
        '<': (-1, 0),
        '>': (+1, 0),
        '^': (0, -1),
        'v': (0, +1),
        }

for y, row in enumerate(tracks):
    for x, ch in enumerate(row):
        if ch in {'<', '>', '^', 'v'}:
            cart = Cart()
            cart.x = x
            cart.y = y
            cart.dx = directions[ch][0]
            cart.dy = directions[ch][1]
            carts.append(cart)

            # fill in track
            tracks[y][x] = '-' if ch in {'<', '>'} else '|'

print(carts)

def check_collisions(c1):
    for c2 in carts:
        if c1 is not c2 and not c2.crashed and (c1.x, c1.y) == (c2.x, c2.y):
            c1.crashed = c2.crashed = True

tick = 1

while sum(map(lambda c: not c.crashed, carts)) > 1:
    carts.sort(key=lambda c: (c.y, c.x))
    for c in carts:
        if c.crashed:
            continue
        x, y = c.x, c.y
        if tracks[y][x] == '+':
            c.turn(next(c.crossroad))
        elif tracks[y][x] in {'/', '\\'}:
            c.bend(tracks[y][x])
        c.advance()
        check_collisions(c)
    tick += 1

pprint(tick)
pprint(carts)
