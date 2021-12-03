require 'set'

data = <<~DATA
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
DATA

data = File.read('../in/day03.in')

def part1(numbers)
  res = numbers.first.zip(*numbers[1..]).map do |a|
    a.sum > numbers.length / 2 ? 1 : 0
  end.join

  gamma   = res.to_i(2)
  epsilon = res.tr('01', '10').to_i(2)

  gamma * epsilon
end

def most_common(bit, numbers)
  numbers.map { |n| n[bit] }.sum < numbers.length.to_f / 2 ? 0 : 1
end

def part2(numbers)
  o2 = numbers.to_set
  co2 = numbers.to_set

  for bit in 0...numbers[0].length
    o2bit = most_common(bit, o2)
    co2bit = 1 - most_common(bit, co2)

    o2.delete_if { |n| n[bit] != o2bit } if o2.size > 1
    co2.delete_if { |n| n[bit] != co2bit } if co2.size > 1
  end

  o2.first.join.to_i(2) * co2.first.join.to_i(2)
end

numbers = data.lines.map { |l| l.strip.split('').map(&:to_i) }

puts part1(numbers)
puts part2(numbers)
