data = <<~EXAMPLE
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
EXAMPLE

data = $stdin.read

elves = data.split("\n\n").map { |e| e.lines.sum(&:to_i) }

puts "Part 1: #{elves.max}"
puts "Part 2: #{elves.max(3).sum}"
