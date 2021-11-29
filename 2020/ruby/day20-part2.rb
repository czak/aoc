require 'scanf'
require 'set'
require 'pry'

Tile = Struct.new(:id, :op, :n, :e, :s, :w)

def parse_tile(tiledata)
  title, grid = tiledata.split("\n", 2)
  id, = title.scanf("Tile %d:")

  n, e, s, w = nil, '', nil, ''
  grid.lines.map(&:strip).each.with_index do |line, i|
    if i == 0
      n = line
    elsif i == 9
      s = line
    end

    w << line[0]
    e << line[9]
  end

  def bin(s)
    s.tr('.#', '01').to_i(2)
  end

  def edgeid(v1, v2)
    min, max = [v1, v2].minmax
    (min << 10) | max
  end

  n = edgeid(bin(n), bin(n.reverse))
  e = edgeid(bin(e), bin(e.reverse))
  s = edgeid(bin(s), bin(s.reverse))
  w = edgeid(bin(w), bin(w.reverse))

  [
    Tile.new(id, :nop, n, e, s, w),
    Tile.new(id, :fx, n, w, s, e),
    Tile.new(id, :fy, s, e, n, w),
    Tile.new(id, :r90, w, n, e, s),
    Tile.new(id, :r90fx, w, s, e, n),
    Tile.new(id, :r90fy, e, n, w, s),
    Tile.new(id, :r180, s, w, n, e),
    Tile.new(id, :r180fx, s, e, n, w),
    Tile.new(id, :r180fy, n, w, s, e),
    Tile.new(id, :r270, e, s, w, n),
    Tile.new(id, :r270fx, e, n, w, s),
    Tile.new(id, :r270fy, w, s, e, n),
  ]
end

def parse(data)
  data.split("\n\n").map do |tiledata|
    parse_tile(tiledata)
  end.flatten
end

def transpose(tiles)
  h = Hash.new { |h, k| h[k] = Set.new }
  tiles.each do |t|
    h[t.n] << t.id
    h[t.e] << t.id
    h[t.s] << t.id
    h[t.w] << t.id
  end
  h
end

# corners: 1693, 1109, 2909, 3371
data = File.read('../in/day20.in')
tiles = parse(data)
edges = transpose(tiles)

GRID_SIZE = 12

raise 'bad edges' if edges.any? { |s| s.size > 2 }

# mark outer edges on tiles as nil
tiles.each do |tile|
  tile.n = nil if edges[tile.n].size == 1
  tile.e = nil if edges[tile.e].size == 1
  tile.s = nil if edges[tile.s].size == 1
  tile.w = nil if edges[tile.w].size == 1
end

grid = Array.new(GRID_SIZE) { Array.new(GRID_SIZE) }
grid[0][0] = tiles.find { |t| t.id == 1109 && t.op == :nop }

def used_ids(grid)
  grid.flatten.compact.map(&:id).to_set
end

for y in 0...GRID_SIZE
  for x in 0...GRID_SIZE
    next if x.zero? && y.zero?

    before = x == 0 ? nil : grid[y][x-1].e
    above  = y == 0 ? nil : grid[y-1][x].s

    grid[y][x] = tiles.find { |t| t.w == before && t.n == above && !used_ids(grid).include?(t.id) }
  end
end

puts "["
grid.each.with_index do |row, y|
  puts "  ["
  row.each.with_index do |tile, x|
    puts "    [#{tile.id}, #{tile.op.inspect.ljust(6)}], "
  end
  puts "  ],"
end
puts "]"
