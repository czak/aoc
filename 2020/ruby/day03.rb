def part1(map)
  traverse(map, 3, 1)
end

def part2(map)
  traverse(map, 1, 1) *
    traverse(map, 3, 1) *
    traverse(map, 5, 1) *
    traverse(map, 7, 1) *
    traverse(map, 1, 2)
end

def traverse(map, dx, dy)
  width = map[0].length

  x = 0
  y = 0
  trees = 0

  while y < map.length
    trees += 1 if map[y][x] == '#'
    x = (x + dx) % width
    y += dy
  end

  trees
end

map = File.readlines("../in/day03.in").map(&:strip)

puts part1(map)
puts part2(map)
