require 'set'

def part1(ids)
  ids.max
end

def part2(ids)
  all = (ids.min...ids.max).to_set
  (all - ids.to_set).first
end

seats = File.readlines("../in/day05.in").map(&:strip)
ids = seats.map do |seat|
  seat.tr('BR', '1').tr('FL', '0').to_i(2)
end

puts part1(ids)
puts part2(ids)
