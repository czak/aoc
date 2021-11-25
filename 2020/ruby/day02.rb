require 'scanf'

def part1(input)
  input.count do |row|
    min, max, ch, pass = row.scanf('%d-%d %c: %s')
    pass.count(ch).between?(min, max)
  end
end

def part2(input)
  input.count do |row|
    i, j, ch, pass = row.scanf('%d-%d %c: %s')
    (pass[i-1] == ch) ^ (pass[j-1] == ch)
  end
end

input = File.readlines("../in/day02.in")

puts "Part 1: #{part1(input)}"
puts "Part 2: #{part2(input)}"
