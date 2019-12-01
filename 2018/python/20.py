from collections import defaultdict
import pdb

def print_rooms(G):
    if not G:
        print('?')
        return
    xlo, xhi = 10e10, -10e10
    ylo, yhi = 10e10, -10e10
    for (x, y) in G.keys():
        if x < xlo: xlo = x
        if x > xhi: xhi = x
        if y < ylo: ylo = y
        if y > yhi: yhi = y
    y = ylo - 0.5
    while y < yhi + 1:
        x = xlo - 0.5
        while x < xhi + 1:
            if x % 1 > 0 and (x+0.5, y) in G[(x-0.5, y)]:
                c = '|'
            elif y % 1 > 0 and (x, y+0.5) in G[(x, y-0.5)]:
                c = '-'
            elif (x, y) == (0, 0):
                c = 'X'
            elif (x, y) in G:
                c = ' '
            else:
                c = '#'
            print(c, end='')
            x += 0.5
        y += 0.5
        print()

def step(pos, direction):
    x, y = pos
    if direction == 'N':
        y -= 1
    elif direction == 'S':
        y += 1
    elif direction == 'W':
        x -= 1
    elif direction == 'E':
        x += 1
    return (x, y)

def find_para(s):
    res = {}
    stack = []
    for i, c in enumerate(s):
        if c == '(':
            stack.append(i)
        elif c == ')':
            res[stack.pop()] = i
    return res

def build_graph(s, start_pos=(0, 0), G=defaultdict(list)):
    curr_pos = start_pos
    para = find_para(s)
    reached = set()
    i = 0
    while i < len(s):
        ch = s[i]
        if ch in '^$':
            i += 1
        elif ch == '|':
            reached.add(curr_pos)
            curr_pos = start_pos
            i += 1
        elif ch == '(':
            last_para = para[i]
            _, j = build_graph(s[i+1:para[i]], curr_pos, G)
            i = last_para+1
        elif ch == ')':
            reached.add(curr_pos)
        else:
            print('reading', s[i:])
            next_pos = step(curr_pos, ch)
            G[curr_pos].append(next_pos)
            G[next_pos].append(curr_pos)
            curr_pos = next_pos
            i += 1
    return G, i

G, _ = build_graph('^E$')
print_rooms(G)

