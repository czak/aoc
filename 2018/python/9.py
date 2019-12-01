from parse import parse

s = '426 players; last marble is worth 7205800 points'

nplayers, max_marble = parse('{:d} players; last marble is worth {:d} points', s)

class Marble:
    def __init__(self, num):
        self.num = num

current = Marble(0)
current.prev = current
current.next = current

player = 0
scores = [0] * nplayers

for i in range(1, max_marble + 1):
    if i % 23 == 0:
        scores[player] += i
        removed = current.prev.prev.prev.prev.prev.prev.prev
        current = removed.next
        removed.prev.next = removed.next
        removed.next.prev = removed.prev
        scores[player] += removed.num
    else:
        left = current.next
        right = current.next.next
        current = Marble(i)
        current.prev = left
        current.next = right
        left.next = current
        right.prev = current
    player = (player + 1) % nplayers

print(max(scores))
