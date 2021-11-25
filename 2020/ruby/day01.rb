def part1(input)
  for i in 0...input.length do
    for j in 0...input.length do
      next if i == j

      a = input[i]
      b = input[j]

      return a * b if a + b == 2020
    end
  end
end

def part2(input)
  for i in 0...input.length do
    for j in 0...input.length do
      for k in 0...input.length do
        next if i == j || i == k || j == k

        a = input[i]
        b = input[j]
        c = input[k]

        return a * b * c if a + b + c == 2020
      end
    end
  end
end

input = File.readlines("../in/day01.in").map(&:to_i)
puts "Part 1: #{part1(input)}"
puts "Part 2: #{part2(input)}"

