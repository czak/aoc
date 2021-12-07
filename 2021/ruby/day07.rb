positions = [16,1,2,0,4,2,7,1,2,14]

positions = File.read('../in/day07.in').split(',').map(&:to_i)

min, max = positions.minmax

part1 = min.upto(max).map do |target|
  positions.sum { |p| (p - target).abs }
end.min

puts part1

part2 = min.upto(max).map do |target|
  positions.sum do |p|
    dist = (p - target).abs
    (dist + dist * dist) / 2
  end
end.min

puts part2
