data = <<~EXAMPLE
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
EXAMPLE

data = $stdin.read

def priority(ch)
  if ch >= 'a'
    ch.ord - 'a'.ord + 1
  else
    ch.ord - 'A'.ord + 27
  end
end

def part1(rucksack)
  s1 = rucksack[...rucksack.length/2].split('')
  s2 = rucksack[rucksack.length/2...].split('')
  priority((s1 & s2).first)
end

def part2(rucksacks)
  s1 = rucksacks[0].split('')
  s2 = rucksacks[1].split('')
  s3 = rucksacks[2].split('')
  priority((s1 & s2 & s3).first)
end

puts "Part 1", data.lines(chomp: true).sum(&method(:part1))
puts "Part 2", data.lines(chomp: true).each_slice(3).sum(&method(:part2))
