require 'scanf'

data = <<~EXAMPLE
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
EXAMPLE

data = $stdin.read

lines = data.lines(chomp: true).map { |l| l.scanf("%d-%d,%d-%d") }

p1 = lines.count do |a, b, c, d|
  a >= c && b <= d || c >= a && d <= b
end

p2 = lines.count do |a, b, c, d|
  a >= c && a <= d ||
  b >= c && b <= d ||
  c >= a && c <= b ||
  d >= a && d <= b
end

puts "Part 1", p1
puts "Part 2", p2
