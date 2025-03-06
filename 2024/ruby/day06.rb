# frozen_string_literal: true

require 'set'

ex1 = <<~EXAMPLE
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
EXAMPLE

class Grid
  def initialize(data)
    @grid = data.split.map(&:strip)
    @rows = @grid.length
    @cols = @grid.first.length
  end

  def find(ch)
    y = @grid.index { |row| row.include?('^') }
    x = @grid[y].index('^')

    [x, y]
  end

  def [](x, y)
    if x < 0 || y < 0 || x >= @cols || y >= @rows
      return nil
    end

    @grid[y][x]
  end

  def []=(x, y, ch)
    if x < 0 || y < 0 || x >= @cols || y >= @rows
      fail
    end

    @grid[y][x] = ch
  end
end

def rotate(dx, dy)
  case [dx, dy]
  when [0, -1] then [1, 0]
  when [1, 0] then [0, 1]
  when [0, 1] then [-1, 0]
  when [-1, 0] then [0, -1]
  end
end

grid = Grid.new($stdin.read)
sx, sy = grid.find('^')

x, y = sx, sy
dx, dy = 0, -1
visited = Set[]

while grid[x, y]
  visited.add([x, y])

  if grid[x+dx, y+dy] == '#'
    dx, dy = rotate(dx, dy)
    next
  end

  x += dx
  y += dy
end

puts visited.length

# part 2

looped = 0

visited.each do |obstacle|
  grid[*obstacle] = '#'

  x, y = sx, sy
  dx, dy = 0, -1
  visited2 = Set[]

  while grid[x, y]
    # have we done this before?
    if visited2.include?([x, y, dx, dy])
      looped += 1
      break
    end

    visited2.add([x, y, dx, dy])

    if grid[x+dx, y+dy] == '#'
      dx, dy = rotate(dx, dy)
      next
    end

    x += dx
    y += dy
  end

  grid[*obstacle] = '.'
end

puts looped
