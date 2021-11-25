grid = <<~DATA.lines.map(&:strip)
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
DATA

grid = File.readlines('../in/day11.in').map(&:strip)

def surroundings(x, y, grid)
  coords = [
    [x-1, y-1],
    [x,   y-1],
    [x+1, y-1],

    [x-1, y  ],
    [x+1, y  ],

    [x-1, y+1],
    [x,   y+1],
    [x+1, y+1],
  ]

  coords.select do |x, y|
    x >= 0 && y >= 0 &&
      x < grid[0].length && y < grid.length
  end
end

def simulate(grid)
  after = Array.new(grid.length) { |_| "." * grid[0].length }

  for y in 0...grid.length
    for x in 0...grid[0].length
      occ = surroundings(x, y, grid).count do |i, j|
        grid[j][i] == '#'
      end

      if grid[y][x] == 'L' && occ == 0
        after[y][x] = '#'
      elsif grid[y][x] == '#' && occ >= 4
        after[y][x] = 'L'
      else
        after[y][x] = grid[y][x]
      end
    end
  end

  after
end

loop do
  after = simulate(grid)
  break if after == grid
  grid = after
end

puts grid.join.count('#')
