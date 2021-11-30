require 'set'

data = <<~DATA
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
DATA

data = File.read('../in/day24.in')

paths = data.lines.map do |line|
  line.strip.scan(/e|se|sw|w|nw|ne/)
end

# which coords are flipped to black
flipped = Set.new

def toggle(flipped, hex)
  if flipped.include?(hex)
    flipped.delete(hex)
  else
    flipped.add(hex)
  end
end

paths.each do |path|
  q, r, s = 0, 0, 0
  path.each do |dir|
    case dir
    when 'e'
      q += 1
      s -= 1
    when 'se'
      r += 1
      s -= 1
    when 'sw'
      q -= 1
      r += 1
    when 'w'
      q -= 1
      s += 1
    when 'nw'
      r -= 1
      s += 1
    when 'ne'
      q += 1
      r -= 1
    else
      raise 'bad dir'
    end
  end

  toggle(flipped, [q, r, s])
end

# Part 1
puts flipped.size

def surroundings(hex)
  q, r, s = hex
  [
    [q+1, r,   s-1],
    [q,   r+1, s-1],
    [q-1, r+1, s],
    [q-1, r,   s+1],
    [q,   r-1, s+1],
    [q+1, r-1, s],
  ]
end

for day in 1..100
  black_to_flip = flipped.select do |hex|
    black_surroundings = (flipped & surroundings(hex)).size
    black_surroundings == 0 || black_surroundings > 2
  end

  white = flipped.map { |hex| surroundings(hex) }.flatten(1).to_set - flipped
  white_to_flip = white.select do |hex|
    black_surroundings = (flipped & surroundings(hex)).size
    black_surroundings == 2
  end

  raise 'bad' if (Set.new(black_to_flip) & Set.new(white_to_flip)).size > 0

  black_to_flip.each { |hex| toggle(flipped, hex) }
  white_to_flip.each { |hex| toggle(flipped, hex) }

  puts "Day #{day}: #{flipped.size}"
end

puts flipped.size
