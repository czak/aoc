from paths import Graph, shortest_path
# from multiprocessing import Pool
# from pprint import pprint

s = """#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"""

s = """#########
#G......#
#.#.....#
#.......#
#...E...#
#.......#
#.......#
#.......#
#########"""

s = """#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"""

s = """#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"""

# s = """#######
# #G..#E#
# #E#E.E#
# #G.##.#
# #...#E#
# #...E.#
# #######"""
#
# s = """#######
# #E..EG#
# #.#G.E#
# #E.##E#
# #G..#.#
# #..E#.#
# #######"""
#
# s = """#######
# #E.G#.#
# #.#G..#
# #G.#.G#
# #G..#.#
# #...E.#
# #######"""

s = """################################
###############..........#######
######.##########G.......#######
#####..###..######...G...#######
#####..#...G..##........########
#####..G......#GG.......########
######..G..#G.......G....#######
########...###...#........######
######....G###.GG#.........#####
######G...####...#..........####
###.##.....G................####
###.......................#.####
##.......G....#####.......E.####
###.......G..#######....##E.####
####........#########..G.#.#####
#.#..##.....#########..#..######
#....####.G.#########......#####
#.##G#####..#########.....###.E#
###########.#########...E.E....#
###########..#######..........##
###########..E#####.......######
###########............E.#######
#########.E.....E..##.#..#######
#######.G.###.......E###########
######...#######....############
################...#############
###############....#############
###############...##############
#################.##############
#################.##############
#################.##############
################################"""

# s = """#########
# #G......#
# #.E.#...#
# #..##..G#
# #...##..#
# #...#...#
# #.G...G.#
# #.....G.#
# #########"""
#
# s = """#######
# #.E...#
# #.#..G#
# #.###.#
# #E#G#G#
# #...#G#
# #######"""

# [(5, 204140, 10), (10, 57226, 3), (20, 61380, 1), (40, 46872, 0), (80, 42826, 0)]
# [(20, 61380, 1), (22, 61380, 1), (25, 55476, 0), (27, 55476, 0), (30, 54434, 0), (32, 54434, 0), (35, 53152, 0), (38, 53152, 0)]
# [(23, 59120, 0), (24, 59120, 0)]


# (ap, remaining_hp, rounds-1, wynik, ilość umarłych elfów)
# [(22, 1395, 44, 61380, 1), (23, 1478, 40, 59120, 0)]

# próby:
# 59120 - too low
# 60598 - too lo

# s = """#######
# #.G...#
# #...EG#
# #.#.#G#
# #..G#E#
# #.....#
# #######"""
#
# s = """#######
# #E..EG#
# #.#G.E#
# #E.##E#
# #G..#.#
# #..E#.#
# #######"""

def printmap(M, elves, goblins):
    for y, row in enumerate(M):
        for x, cell in enumerate(row):
            drawn = False
            for e in elves:
                if e.hp <= 0: continue
                if (e.x, e.y) == (x, y):
                    print('E', end='')
                    drawn = True
                    break
            for g in goblins:
                if g.hp <= 0: continue
                if (g.x, g.y) == (x, y):
                    print('G', end='')
                    drawn = True
                    break
            if not drawn:
                print(cell, end='')
        print()

class Unit:
    def __init__(self, t, x, y, enemies, ap):
        self.t = t
        self.x, self.y = x, y
        self.hp = 200
        self.ap = ap
        self.enemies = enemies
    def isalive(self):
        return self.hp > 0
    def pos(self):
        return (self.x, self.y)
    def can_reach(self, pos):
        visited = set()
        def fill(x, y):
            if isopen(x, y) and (x, y) not in visited:
                visited.add((x, y))
                fill(x, y-1)
                fill(x-1, y)
                fill(x+1, y)
                fill(x, y+1)
        fill(self.x, self.y-1)
        fill(self.x-1, self.y)
        fill(self.x+1, self.y)
        fill(self.x, self.y+1)
        return pos in visited
    def range(self):
        return set((
            (self.x  , self.y-1), 
            (self.x-1, self.y  ), 
            (self.x+1, self.y  ), 
            (self.x  , self.y+1), 
            ))
    def __repr__(self):
        return "{.t}({.x},{.y}):{.hp}".format(self)

# printmap(M, elves, goblins)


def prepare_graph(M, units, current_unit):
    alive_unit_positions = set(u.pos() for u in units - set([current_unit]) if u.hp > 0)
    g = Graph()
    for y, row in enumerate(M):
        for x, cell in enumerate(row):
            if cell == '.' and (x, y) not in alive_unit_positions:
                g.add_vertex((x, y))
    for v in g.vertices:
        x, y = v
        if (x-1,  y) in g.vertices:
            g.add_edge((x, y), (x-1, y), 1)
        if (x+1,  y) in g.vertices:
            g.add_edge((x, y), (x+1, y), 1)
        if (x,  y-1) in g.vertices:
            g.add_edge((x, y), (x, y-1), 1)
        if (x,  y+1) in g.vertices:
            g.add_edge((x, y), (x, y+1), 1)
    return g


def isopen(M, units, x, y, ignore=set()):
    if M[y][x] == '#':
        return False
    alive_unit_positions = set(u.pos() for u in units - ignore if u.hp > 0)
    if (x, y) in alive_unit_positions:
        return False
    return True

def simulate(ap):
    M = [list(row) for row in s.splitlines()]

    elves = set()
    goblins = set()

    for y, row in enumerate(M):
        for x, cell in enumerate(row):
            if cell == 'G':
                goblins.add(Unit('G', x, y, enemies=elves, ap=3))
                row[x] = '.'
            elif cell == 'E':
                elves.add(Unit('E', x, y, enemies=goblins, ap=ap))
                row[x] = '.'

    rounds = 0
    finished = False
    while not finished:
        # wszystkie jednostki zebrać i posortować
        units = sorted(elves | goblins, key=lambda unit: (unit.y, unit.x))

        # print(units)

        # dla każdej jednostki:
        for u in units:
            # print(u)
            # jeśli już nie żyję - koniec tury
            if u.hp <= 0:
                continue

            alive_enemies = set(filter(Unit.isalive, u.enemies))
            if not alive_enemies:
                # => koniec pojedynku
                # print("BREAK!!!!!")
                finished = True
                break

            # sprawdź czy jestem w zasięgu jakiegoś żywego wroga
            enemies_in_range = set(e for e in alive_enemies if e.pos() in u.range())

            # jeśli nie
            if not enemies_in_range:
                target_positions = set(pos for e in alive_enemies for pos in e.range() if isopen(M, (elves | goblins), pos[0], pos[1]))
                # print('target pos:', target_positions)
                if target_positions:
                    graph = prepare_graph(M, (elves | goblins), u)
                    paths = [shortest_path(graph, u.pos(), pos) for pos in target_positions]
                    paths = [p for p in paths if u.pos() in p]
                    if not paths:
                        continue
                    # wybierz najbliższy punkt - target
                    shortest_path_length = min(map(len, paths))
                    nearest_targets = [path[-1] for path in paths if len(path) == shortest_path_length]
                    final_target = min(nearest_targets, key=lambda p: (p[1], p[0]))
                    # w którą stronę następny krok
                    paths = [shortest_path(graph, pos, final_target) for pos in u.range() if isopen(M, (elves | goblins), pos[0], pos[1])]
                    # print(paths)
                    shortest_path_length = min(map(len, paths))
                    # print(shortest_path_length)
                    best_start_points = [path[0] for path in paths if len(path) == shortest_path_length] 
                    # print(best_start_points)
                    final_start_point = min(best_start_points, key=lambda p: (p[1], p[0]))
                    # print(final_start_point)
                    u.x, u.y = final_start_point

            # jeśli jestem w zasięgu jakiegoś żywego wroga
            alive_enemies = set(filter(Unit.isalive, u.enemies))
            enemies_in_range = set(e for e in alive_enemies if e.pos() in u.range())
            if enemies_in_range:
                lowest_hp = min(map(lambda u: u.hp, enemies_in_range))
                weakest_enemies = [e for e in enemies_in_range if e.hp == lowest_hp]
                final_enemy = min(weakest_enemies, key=lambda u: (u.y, u.x))
                final_enemy.hp -= u.ap

        rounds += 1
        # printmap(M, elves, goblins)

    remaining_hp = sum(u.hp for u in elves | goblins if u.hp > 0)
    # print(ap, remaining_hp * (rounds - 1))
    elves_dead = sum(elf.hp <= 0 for elf in elves)
    return(ap, remaining_hp, rounds-1, remaining_hp * (rounds - 1), elves_dead)

if __name__ == '__main__':
    simulate(16)


# (5, 204140, 10),
# (10, 57226, 3),
# (20, 61380, 1),
# (22, 61380, 1),
# (23, 59120, 0), => mój typ
# (24, 59120, 0)
# (25, 55476, 0),
# (27, 55476, 0),
# (30, 54434, 0),
# (32, 54434, 0),
# (35, 53152, 0),
# (38, 53152, 0)
# (40, 46872, 0),
# (80, 42826, 0)]
