require 'set'
require 'scanf'

data = <<~DATA
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
DATA

data = File.read('../in/day13.in')

coords = data.split("\n\n")[0].lines.map { _1.scanf('%d,%d') }
folds = data.split("\n\n")[1].lines.map { _1.scanf('fold along %c=%d') }

xmax = coords.map(&:first).max
ymax = coords.map(&:last).max

cols = Array.new(xmax+1) { Set[] }
rows = Array.new(ymax+1) { Set[] }

coords.each do |x, y|
  cols[x] << y
  rows[y] << x
end

def fold(rows, cols, along, n)
  if along == 'y'
    0.upto(n-1) { |y| rows[y] |= rows[2*n-y] }
    cols.each do |col|
      col.select { |m| m > n }.each { |m| col << 2*n-m }
    end
  else
    0.upto(n-1) { |x| cols[x] |= cols[2*n-x] }
    rows.each do |row|
      row.select { |m| m > n }.each { |m| row << 2*n-m }
    end
  end
end

def part1(rows, cols, folds)
  along, n = folds.first

  fold(rows, cols, along, n)

  if along == 'y'
    rows[0...n].sum(&:size)
  else
    cols[0...n].sum(&:size)
  end
end

def part2(rows, cols, folds)
  width = cols.length
  height = rows.length

  folds.each do |along, n|
    fold(rows, cols, along, n)
    if along == 'x'
      width = n
    else
      height = n
    end
  end

  0.upto(height-1).map do |y|
    0.upto(width-1).map do |x|
      rows[y].include?(x) ? '#' : '.'
    end.join
  end.join("\n")
end

puts part1(rows, cols, folds)
puts part2(rows, cols, folds)
