# example
p1, p2 = 4, 8

# input
p1, p2 = 4, 1

def part1(p1, p2)
  s1, s2 = 0, 0
  rolls = 0
  die = 1.upto(100).cycle.each_slice(3).lazy.map(&:sum)

  loop do
    roll = die.next
    p1 = (p1 + roll) % 10
    s1 = s1 + p1 + 1
    rolls += 1

    return rolls * 3 * s2 if s1 >= 1000

    roll = die.next
    p2 = (p2 + roll) % 10
    s2 = s2 + p2 + 1
    rolls += 1

    return rolls * 3 * s1 if s2 >= 1000
  end
end

puts part1(p1-1, p2-1)
