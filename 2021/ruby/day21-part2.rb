# example
p1, p2 = 4, 8

# input
p1, p2 = 4, 1

ROLLS = [1, 2, 3].product([1, 2, 3]).product([1, 2, 3]).map { |a| a.flatten.sum }.freeze

# See https://github.com/mebeim/aoc/blob/master/2021/README.md#part-2-19
def part2(pa, pb, sa, sb, cache)
  return cache[[pa, pb, sa, sb]] if cache.include?([pa, pb, sa, sb])

  return [1, 0] if sa >= 21
  return [0, 1] if sb >= 21

  wa, wb = 0, 0

  ROLLS.each do |roll|
    p = (pa + roll) % 10
    s = sa + p + 1

    b, a = part2(pb, p, sb, s, cache)

    wa += a
    wb += b
  end

  cache[[pa, pb, sa, sb]] = [wa, wb]
end

puts part2(p1-1, p2-1, 0, 0, {}).max
