from parse import parse
from pprint import pprint

lines = open('10.in').read().splitlines()

coords = [list(parse('position=<{:>d}, {:>d}> velocity=<{:>d},{:>d}>', line).fixed) for line in lines]

def bounds():
    xlo = None
    ylo = None
    xhi = None
    yhi = None
    for x, y, *_ in coords:
        if not xlo or x < xlo: xlo = x
        if not xhi or x > xhi: xhi = x
        if not ylo or y < ylo: ylo = y
        if not yhi or y > yhi: yhi = y
    return (xlo, ylo, xhi, yhi)

def surface_area():
    xlo, ylo, xhi, yhi = bounds()
    return (xhi-xlo) * (yhi-ylo)

previous_area = 10e20

for second in range(1, 10868):
    for c in coords:
        x, y, dx, dy = c
        c[0] += dx
        c[1] += dy
    current_area = surface_area()
    if current_area > previous_area:
        print(second)
        break
    previous_area = current_area

def print_grid():
    xlo, ylo, xhi, yhi = bounds() 
    grid = [bytearray(b'.' * (xhi-xlo+1)) for y in range(yhi - ylo + 1)]
    for x, y, *_ in coords:
        a = x - xlo
        b = y - ylo
        grid[b][a] = ord('#')
    print('\n'.join(map(bytearray.decode, grid)))

print_grid()

# for second in range(5):
#     for c in coords:
#         x, y, dx, dy = c
#         c[0] -= dx
#         c[1] -= dy
#     print_grid()

