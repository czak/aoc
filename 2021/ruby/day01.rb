def part1(measurements)
  measurements.each_cons(2).count { |a, b| b > a }
end

def part2(measurements)
  measurements.each_cons(3).map(&:sum).each_cons(2).count { |a, b| b > a }
end

measurements = File.readlines('../in/day01.in').map(&:to_i)

puts part1(measurements)
puts part2(measurements)
