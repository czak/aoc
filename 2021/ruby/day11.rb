data = <<~DATA
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
DATA

data = File.read('../in/day11.in')

grid = data.lines(chomp: true).map do |line|
  line.chars.map(&:to_i)
end

def step(grid)
  width = grid.first.length
  height = grid.length

  flash = []
  done = []

  excite = -> ((x, y)) do
    return if x < 0 || y < 0 || x >= width || y >= height
    val = grid[y][x]
    flash << [x, y] if val == 9
    grid[y][x] = val + 1
  end

  xs = (0...height).to_a
  ys = (0...width).to_a
  xs.product(ys).each(&excite)

  done << flash.shift.tap do |x, y|
    [x-1, x, x+1].product([y-1, y, y+1]).each(&excite)
  end until flash.empty?

  done.each { |x, y| grid[y][x] = 0 }
end

def part1(grid)
  100.times.sum { step(grid).length }
end

def part2(grid)
  101.upto(1000) do |n|
    return n if step(grid).length == grid[0].length * grid.length
  end
end

puts part1(grid)
puts part2(grid)
