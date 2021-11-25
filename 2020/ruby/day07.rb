require 'set'

data = File.readlines('../in/day07.in').map do |line|
  outer, inner = line.strip[0...-1].split(' contain ')
  [
    outer[0...-5], # without " bags"
    inner.split(', ').map do |i|
      [
        i[0].to_i,
        i.end_with?('s') ? i[2...-5] : i[2...-4] # without " bag" or " bags",
      ]
    end
  ]
end

def part1(data)
  h = Hash.new { |h, k| h[k] = Set.new }

  data.each do |outer_bag, inner_bags|
    inner_bags.each do |n, inner_bag|
      h[inner_bag].add(outer_bag)
    end
  end

  def collect(h, bag)
    h[bag] | h[bag].reduce(Set.new) { |s, b| s | collect(h, b) }
  end

  collect(h, 'shiny gold').size
end

def part2(data)
  h = Hash.new { |h, k| h[k] = [] }

  data.each do |outer_bag, inner_bags|
    inner_bags.each do |n, inner_bag|
      n.times do
        h[outer_bag] << inner_bag
      end
    end
  end

  def collect(h, bag)
    h[bag] + h[bag].reduce([]) { |a, b| a + collect(h, b) }
  end

  collect(h, 'shiny gold').size
end

puts part1(data)
puts part2(data)
