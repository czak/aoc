from multiprocessing import Pool

def power_level(x, y, serial_number):
    rack_id = x + 10
    num = (rack_id * y + serial_number) * rack_id
    if num < 100:
        num = 0
    else:
        num = int(str(num)[-3])
    return num - 5

# print(power_level(122, 79, serial_number=57))
# print(power_level(217, 196, serial_number=39))
# print(power_level(101,153, serial_number=71))

# hi = (-1, -1, -1, -1e10)
serial_number = 8141

grid = [[0] * 301 for _ in range(301)]
totals = [[{} for _ in range(301)] for _ in range(301)]

for x in range(1, 301):
    for y in range(1, 301):
        grid[x][y] = power_level(x, y, serial_number)

# def size_best(size):
#     print(size)
#     size_hi = (-1, -1, -1, -1e10)
#     for x in range(1, 300 - size + 1):
#         for y in range(1, 300 - size + 1):
#             cells = ((a, b) for a in range(x, x+size) for b in range(y, y+size))
#             total = sum(map(lambda c: grid[c[0]][c[1]], cells))
#             if total > size_hi[3]:
#                 size_hi = (x, y, size, total)
#     return size_hi

for x in range(1, 301):
    for y in range(1, 301):
        totals[x][y][1] = grid[x][y]

for size in range(2, 301):
    print(size)
    for x in range(1, 300 - size + 1):
        for y in range(1, 300 - size + 1):
            new_cells = {(a, y+size-1) for a in range(x, x+size)} | {(x+size-1, b) for b in range(y, y+size)}
            totals[x][y][size] = totals[x][y][size-1] + sum(map(lambda c: grid[c[0]][c[1]], new_cells))


hi = (-1, -1, -1, -1e10)
for size in range(1, 17):
    for x in range(1, 300 - size + 1):
        for y in range(1, 300 - size + 1):
            total = totals[x][y][size]
            if total > hi[3]:
                hi = (x, y, size, total)

print(hi)
