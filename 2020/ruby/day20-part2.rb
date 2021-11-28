require 'scanf'
require 'set'

Tile = Struct.new(:id, :n, :e, :s, :w)

def edgeid(v1, v2)
  min, max = [v1, v2].minmax
  (min << 10) | max
end

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

  [
    id,
    Tile.new(
      id,
      edgeid(bin(n), bin(n.reverse)),
      edgeid(bin(e), bin(e.reverse)),
      edgeid(bin(s), bin(s.reverse)),
      edgeid(bin(w), bin(w.reverse)),
    ),
  ]
end

def parse(data)
  data.split("\n\n").map do |tiledata|
    parse_tile(tiledata)
  end.to_h
end

def transpose(tiles)
  h = Hash.new { |h, k| h[k] = Set.new }
  tiles.each do |_, t|
    h[t.n] << t
    h[t.e] << t
    h[t.s] << t
    h[t.w] << t
  end
  h
end

# corners: 1693, 1109, 2909, 3371
data = File.read('../in/day20.in')

tiles = parse(data)
edges = transpose(tiles)

tile = tiles[1693]

matched = edges.select { |e, _| tile.e == e }

puts matched.inspect
