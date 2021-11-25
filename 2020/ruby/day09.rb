def valid?(n, preamble)
  for i in 0...preamble.length
    for j in 0...preamble.length
      next if i == j
      return true if n == preamble[i] + preamble[j]
    end
  end
  false
end

def part1(numbers, preamble_size:)
  preamble_size.upto(numbers.length - 1) do |offset|
    current = numbers[offset]
    preamble = numbers[offset-preamble_size...offset]

    return current unless valid?(current, preamble)
  end
end

def part2(numbers, total)
  0.upto(numbers.length - 2) do |l|
    sum = numbers[l]
    (l+1).upto(numbers.length - l) do |r|
      sum += numbers[r]
      return numbers[l..r].min + numbers[l..r].max if sum == total
      break if sum > total
    end
  end
end

data = File.readlines('../in/day09.in').map(&:to_i)
# data = <<~DATA.lines.map(&:to_i)
# 35
# 20
# 15
# 25
# 47
# 40
# 62
# 55
# 65
# 95
# 102
# 117
# 150
# 182
# 127
# 219
# 299
# 277
# 309
# 576
# DATA

invalid = part1(data, preamble_size: 25)
puts invalid
puts part2(data, invalid)
