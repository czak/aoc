def part1(chain)
  n1 = 0
  n3 = 0

  chain.each_cons(2) do |from, to|
    case to - from
    when 1 then n1 += 1
    when 3 then n3 += 1
    end
  end

  n1 * n3
end

def part2(chain)
  routes = [1, 1, chain[2] <= 3 ? 2 : 1]

  chain.each_cons(4) do |a, b, c, d|
    sum = 0
    sum += routes[0] if d - a <= 3
    sum += routes[1] if d - b <= 3
    sum += routes[2]

    routes.push(sum)
    routes.shift
  end

  routes.last
end

adapters = File.readlines('../in/day10.in').map(&:to_i)
outlet   = 0
device   = adapters.max + 3

chain = [outlet, *adapters.sort, device]

puts part1(chain)
puts part2(chain)
