data = <<~DATA
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
DATA

data = File.read('../in/day14.in')

lines = data.lines(chomp: true)

map = lines.first.chars.each_cons(2).map(&:join).tally
rules = lines[2..].map do |line|
  from, to = line.split(' -> ')
  [from, ["#{from[0]}#{to}", "#{to}#{from[1]}"]]
end.to_h

def step(map, rules)
  h = Hash.new(0)
  map.each do |pair, count|
    a, b = rules[pair]
    h[a] += count
    h[b] += count
  end
  h
end

def totals(map)
  h = Hash.new(0)
  map.each do |pair, count|
    h[pair[0]] += count
    h[pair[1]] += count
  end
  h.transform_values { |count| count.divmod(2).sum }
end

def transform(map, rules, num)
  num.times { map = step(map, rules) }
  min, max = totals(map).values.minmax
  max - min
end

puts transform(map, rules, 10)
puts transform(map, rules, 40)
