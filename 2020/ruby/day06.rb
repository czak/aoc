require 'set'

def part1(groups)
  groups.sum do |g|
    g.join.gsub(/\s/, '').split('').to_set.size
  end
end

def part2(groups)
  groups.sum do |g|
    g.map { |l| l.split('').to_set }.reduce(:intersection).size
  end
end

groups = File.read("../in/day06.in").split("\n\n").map do |block|
  block.split(/\s/)
end

puts part1(groups)
puts part2(groups)
