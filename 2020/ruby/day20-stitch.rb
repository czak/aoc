require 'scanf'
require 'colorize'

MAP = [
  [
    [1109, :nop  ], 
    [1181, :fy   ], 
    [2719, :fx   ], 
    [1373, :r270 ], 
    [1303, :r270 ], 
    [1637, :fx   ], 
    [3023, :r270 ], 
    [1447, :r90fx], 
    [1361, :r90  ], 
    [1019, :r270 ], 
    [1733, :fx   ], 
    [1693, :r90fy], 
  ],
  [
    [1487, :r90fy], 
    [2441, :nop  ], 
    [3671, :r90fx], 
    [3221, :r270 ], 
    [1367, :nop  ], 
    [1097, :r270 ], 
    [3761, :fy   ], 
    [1601, :r90fx], 
    [1091, :fy   ], 
    [3253, :nop  ], 
    [1667, :r90  ], 
    [1609, :fy   ], 
  ],
  [
    [3319, :r90fy], 
    [3257, :r270 ], 
    [3863, :fy   ], 
    [2593, :nop  ], 
    [3089, :r90  ], 
    [3181, :r90fx], 
    [2273, :r90  ], 
    [2699, :nop  ], 
    [1429, :fx   ], 
    [1697, :fy   ], 
    [3691, :r90fy], 
    [2861, :nop  ], 
  ],
  [
    [2423, :r270 ], 
    [1213, :fy   ], 
    [1559, :r90fy], 
    [2969, :r270 ], 
    [2621, :r90  ], 
    [3739, :r270 ], 
    [1193, :r270 ], 
    [1823, :fx   ], 
    [2531, :fx   ], 
    [1489, :r90  ], 
    [2549, :fy   ], 
    [1867, :r90fy], 
  ],
  [
    [2161, :fy   ], 
    [3019, :r90fx], 
    [3881, :r90fx], 
    [2411, :nop  ], 
    [1619, :r90  ], 
    [2081, :nop  ], 
    [2617, :nop  ], 
    [2707, :fy   ], 
    [3527, :r90fx], 
    [3467, :r90fy], 
    [1439, :r90fy], 
    [3229, :r90  ], 
  ],
  [
    [2999, :fx   ], 
    [2843, :r90fx], 
    [2083, :fx   ], 
    [1789, :fy   ], 
    [2131, :fx   ], 
    [2521, :fy   ], 
    [3137, :r90fx], 
    [3673, :r90fx], 
    [2377, :r90fy], 
    [1123, :r90  ], 
    [1847, :r90fy], 
    [3313, :nop  ], 
  ],
  [
    [1069, :r90fy], 
    [2557, :r270 ], 
    [2777, :r90fx], 
    [1049, :r270 ], 
    [1993, :fy   ], 
    [1423, :fx   ], 
    [3931, :r270 ], 
    [1153, :fx   ], 
    [2663, :r90fy], 
    [1201, :r90fy], 
    [1129, :r90fx], 
    [3677, :r90fx], 
  ],
  [
    [2371, :nop  ], 
    [1553, :fx   ], 
    [3727, :r90fy], 
    [1249, :r90fy], 
    [1607, :r270 ], 
    [1741, :r90fy], 
    [2683, :r270 ], 
    [2039, :r90fx], 
    [1327, :nop  ], 
    [3011, :r90fy], 
    [1033, :r90  ], 
    [3833, :r90  ], 
  ],
  [
    [3821, :fx   ], 
    [3491, :r90fx], 
    [1783, :fy   ], 
    [3877, :fx   ], 
    [3413, :r90  ], 
    [3659, :r90fx], 
    [3517, :nop  ], 
    [1879, :r90  ], 
    [3583, :r270 ], 
    [3191, :r90  ], 
    [3533, :r90fy], 
    [3779, :r90fy], 
  ],
  [
    [1523, :r90fx], 
    [3329, :fy   ], 
    [2677, :nop  ], 
    [2953, :r270 ], 
    [3463, :r270 ], 
    [1171, :fy   ], 
    [1627, :r270 ], 
    [2789, :r90fy], 
    [3203, :r90fy], 
    [3461, :fx   ], 
    [1103, :r270 ], 
    [2819, :fx   ], 
  ],
  [
    [3637, :fx   ], 
    [1997, :r90  ], 
    [2221, :r90fy], 
    [2467, :r270 ], 
    [2287, :r90fy], 
    [2551, :r90fy], 
    [2897, :r90  ], 
    [1747, :r90  ], 
    [3967, :r90fy], 
    [1657, :r90  ], 
    [3299, :fx   ], 
    [2917, :r90fx], 
  ],
  [
    [3371, :fx   ], 
    [2543, :fy   ], 
    [2749, :r90fx], 
    [3301, :nop  ], 
    [2141, :fy   ], 
    [1061, :nop  ], 
    [1597, :r90fy], 
    [2459, :fx   ], 
    [1759, :r90fx], 
    [2633, :fx   ], 
    [2207, :r90fx], 
    [2909, :fy   ], 
  ],
]

def parse_tile(tiledata)
  title, grid = tiledata.split("\n", 2)

  id = title.scanf("Tile %d:").first
  grid = grid.lines.map do |line|
    line.strip.split('')
  end

  [id, grid]
end

def parse(data)
  data.split("\n\n").map do |tiledata|
    parse_tile(tiledata)
  end.to_h
end

TILES = parse(File.read('../in/day20.in'))
GRID = Array.new(96) { Array.new(96, ' ') }

def place(tile, ox, oy)
  for y in 0...8
    for x in 0...8
      GRID[oy+y][ox+x] = tile[y+1][x+1]
    end
  end
end

def mangle(tile, op)
  case op
  when :nop
    tile
  when :fy
    tile.reverse
  when :fx
    tile.map(&:reverse)
  when :r270
    tile.transpose.reverse
  when :r90
    tile.transpose.map(&:reverse)
  when :r90fx
    tile.transpose
  when :r90fy
    tile.transpose.map(&:reverse).reverse
  else
    raise "unknown #{op}"
  end
end

for y in 0...12
  for x in 0...12
    id, op = MAP[y][x]
    place(mangle(TILES[id], op), x*8, y*8)
  end
end

puts GRID.map(&:join).join("\n")
