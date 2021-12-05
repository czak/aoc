require 'scanf'
require 'set'

data = <<~DATA
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
DATA

data = File.read('../in/day05.in')

Point = Struct.new(:x, :y)
Line = Struct.new(:a, :b)

Line.define_method(:vertical?) { a.x == b.x }
Line.define_method(:horizontal?) { a.y == b.y }

lines = data.lines.map do |l|
  x1, y1, x2, y2 = l.scanf('%d,%d -> %d,%d')
  Line.new(
    Point.new(x1, y1),
    Point.new(x2, y2),
  )
end

grid = Set.new
duplicates = Set.new

plot = -> (x, y) do
  p = Point.new(x, y)
  duplicates.add(p) if grid.include?(p)
  grid.add(p)
end

def part1(lines, plot)
  lines.select(&:vertical?).each do |line|
    x = line.a.x
    y1, y2 = [line.a.y, line.b.y].minmax

    y1.upto(y2) { |y| plot.call(x, y) }
  end

  lines.select(&:horizontal?).each do |line|
    x1, x2 = [line.a.x, line.b.x].minmax
    y = line.a.y

    x1.upto(x2) { |x| plot.call(x, y) }
  end
end

def part2(lines, plot)
  lines.reject { _1.horizontal? || _1.vertical? }.each do |line|
    a, b = [line.a, line.b].minmax_by(&:y)
    x = a.x

    (a.y).upto(b.y) do |y|
      plot.call(x, y)
      x += if x < b.x then 1 else -1 end
    end
  end
end

part1(lines, plot)
puts duplicates.size

part2(lines, plot)
puts duplicates.size
