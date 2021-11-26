def part1(ts, schedule)
  ids = schedule.map(&:first)

  delays = ids.map do |id|
    id - (ts % id)
  end

  id, delay = ids.zip(delays).min_by(&:last)

  id * delay
end

def find(start, divider, offset, jump)
  n = start
  loop do
    return n if (n + offset) % divider == 0
    n += jump
  end
  n
end

def part2(ts, schedule)
  current = 0
  multiplier = 1

  schedule.each do |id, offset|
    current = find(current, id, offset, multiplier)
    multiplier *= id
  end

  current
end

data = File.read('../in/day13.in')

# data = <<~DATA
# 939
# 7,13,x,x,59,x,31,19
# DATA

ts = data.lines[0].to_i
schedule = data.lines[1].split(',').map(&:to_i)
  .map.with_index.select { |id, offset| id > 0 }

puts part1(ts, schedule)
puts part2(ts, schedule)


