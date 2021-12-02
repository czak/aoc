def part1(instructions)
  x, d = 0, 0
  instructions.each do |dir, dist|
    case dir
    when :forward
      x += dist
    when :down
      d += dist
    when :up
      d -= dist
    end
  end
  x * d
end

def part2(instructions)
  x, d, aim = 0, 0, 0
  instructions.each do |dir, val|
    case dir
    when :forward
      x += val
      d += val * aim
    when :down
      aim += val
    when :up
      aim -= val
    end
  end
  x * d
end

data = File.read('../in/day02.in')
instructions = data.lines.map do |line|
  dir, dist = line.split
  [dir.to_sym, dist.to_i]
end

puts part1(instructions)
puts part2(instructions)
