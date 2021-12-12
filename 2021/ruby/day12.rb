require 'set'

data = <<~DATA
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
DATA

data = File.read('../in/day12.in')

map = Hash.new { |h, k| h[k] = Set[] }

data.lines(chomp: true).each do |line|
  a, b = line.split('-')
  map[a] << b unless a == 'end' || b == 'start'
  map[b] << a unless b == 'end' || a == 'start'
end

def part1(map, from, to, visited)
  return 1 if from == to

  map[from].sum do |b|
    if b.match(/[a-z]/) && visited.include?(b)
      0
    else
      part1(map, b, to, visited | [from])
    end
  end
end

def part2(map, from, to, visited, twice = false)
  return 1 if from == to

  if from.match(/[a-z]/)
    twice = true if visited.include?(from)
    visited |= [from]
  end

  map[from].sum do |b|
    if twice && visited.include?(b)
      0
    else
      part2(map, b, to, visited, twice)
    end
  end
end

puts part1(map, 'start', 'end', Set[])
puts part2(map, 'start', 'end', Set[])
