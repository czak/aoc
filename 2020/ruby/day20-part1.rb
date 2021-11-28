require 'scanf'
require 'set'

# => [id, [n, n', e, e', s, s', w, w']]
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
    [
      bin(n), bin(n.reverse),
      bin(e), bin(e.reverse),
      bin(s), bin(s.reverse),
      bin(w), bin(w.reverse),
    ]
  ]
end

def parse(data)
  data.split("\n\n").map do |tiledata|
    parse_tile(tiledata)
  end.to_h
end

data = File.read('../in/day20.in')

tiles = parse(data)

corner_tiles = tiles.select do |id, edges|
  # find edges of all *other* tiles
  other_edges = tiles.reject { |tid, _| tid == id }.values.flatten

  # subtract from mine
  left = edges - other_edges

  # corner tiles will have 4 left
  left.length == 4
end

puts "corners: #{corner_tiles.keys.join(', ')}"

puts corner_tiles.keys.reduce(:*)
