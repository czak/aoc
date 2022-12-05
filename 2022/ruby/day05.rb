require 'scanf'

data = <<~EXAMPLE
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
EXAMPLE

data = $stdin.read

def parse(data)
  stacks = data.split("\n\n")[0].lines(chomp: true)
    .reverse
    .map(&:chars)
    .transpose
    .filter { |a| a[0].to_i > 0 }
    .map { |a| a.filter { |ch| ch != " " }[1..] }

  instructions = data.split("\n\n")[1].lines(chomp: true)
    .map { |i| i.scanf("move %d from %d to %d") }
    .map { |c, s, t| [c, s-1, t-1] }

  [stacks, instructions]
end

def part1(stacks, instructions)
  instructions.each do |count, source, target|
    count.times { stacks[target].push(stacks[source].pop) }
  end
  stacks.map(&:last).join
end

def part2(stacks, instructions)
  instructions.each do |count, source, target|
    tmp = []
    count.times { tmp.push(stacks[source].pop) }
    count.times { stacks[target].push(tmp.pop) }
  end
  stacks.map(&:last).join
end

puts "Part 1", part1(*parse(data))
puts "Part 2", part2(*parse(data))
