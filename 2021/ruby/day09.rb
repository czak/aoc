require 'set'

data = <<~DATA
2199943210
3987894921
9856789892
8767896789
9899965678
DATA

data = File.read('../in/day09.in')

grid = data.lines.map do |line|
  line.strip.chars.map(&:to_i)
end

def low_points(grid)
  width = grid[0].length
  height = grid.length

  res = []

  for y in 0...height
    for x in 0...width
      current = grid[y][x]
      next if x > 0            && grid[y][x-1] <= current
      next if x < (width - 1)  && grid[y][x+1] <= current
      next if y > 0            && grid[y-1][x] <= current
      next if y < (height - 1) && grid[y+1][x] <= current
      res << [x, y]
    end
  end

  res
end

def part1(grid)
  low_points(grid).reduce(0) do |sum, (x, y)|
    sum += 1 + grid[y][x]
  end
end

def fill(grid, x, y, visited = Set.new)
  return visited if grid[y][x] == 9 || visited.include?([x, y])

  visited.add([x, y])

  fill(grid, x-1, y, visited) if x > 0
  fill(grid, x+1, y, visited) if x < grid[0].length - 1
  fill(grid, x, y-1, visited) if y > 0
  fill(grid, x, y+1, visited) if y < grid.length - 1

  visited
end

def part2(grid)
  low_points(grid)
    .map { |x, y| fill(grid, x, y).size }
    .max(3)
    .reduce(:*)
end

puts part1(grid)
puts part2(grid)
