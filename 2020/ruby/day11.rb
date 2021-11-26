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

def simulate(grid, occ_threshold:)
  after = Array.new(grid.length) { |_| "." * grid[0].length }

  for y in 0...grid.length
    for x in 0...grid[0].length
      next if grid[y][x] == '.'

      occ = yield x, y

      if grid[y][x] == 'L' && occ == 0
        after[y][x] = '#'
      elsif grid[y][x] == '#' && occ >= occ_threshold
        after[y][x] = 'L'
      else
        after[y][x] = grid[y][x]
      end
    end
  end

  after
end

def part1(grid)
  loop do
    after = simulate(grid, occ_threshold: 4) do |x, y|
      surroundings(x, y, grid).count do |i, j|
        grid[j][i] == '#'
      end
    end
    break if after == grid
    grid = after
  end
  grid.join.count('#')
end

def line_occurrences(x, y, grid)
  deltas = [
    [-1, -1],
    [ 0, -1],
    [ 1, -1],
    [-1,  0],
    [ 1,  0],
    [-1,  1],
    [ 0,  1],
    [ 1,  1],
  ]

  width = grid[0].length
  height = grid.length

  occ = 0
  deltas.each do |dx, dy|
    cx, cy = x, y
    loop do
      cx += dx
      cy += dy

      # Reached edge
      if cx < 0 || cy < 0 || cx >= width || cy >= height
        break
      end

      tile = grid[cy][cx]

      if tile == '#'
        occ += 1
        break
      elsif tile == 'L'
        break
      end
    end
  end

  occ
end

def part2(grid)
  loop do
    after = simulate(grid, occ_threshold: 5) do |x, y|
      line_occurrences(x, y, grid)
    end
    break if after == grid
    grid = after
  end
  grid.join.count('#')
end

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

puts part1(grid)
puts part2(grid)
