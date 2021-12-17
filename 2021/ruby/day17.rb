require 'scanf'

data = 'target area: x=20..30, y=-10..-5'
data = File.read('../in/day17.in')

target = Struct
  .new(:xmin, :xmax, :ymin, :ymax)
  .new(*data.scanf('target area: x=%d..%d, y=%d..%d'))

def target.includes?(x, y)
  x >= xmin && x <= xmax &&
    y >= ymin && y <= ymax
end

def part1(target)
  vy = -target.ymin - 1
  (1 + vy) / 2 * vy
end

def part2(target)
  vymin = target.ymin
  vymax = -target.ymin - 1
  vxmin = ((Math.sqrt(8 * target.xmin + 1) - 1) / 2).ceil
  vxmax = target.xmax

  count = 0

  vymin.upto(vymax) do |svy|
    vxmin.upto(vxmax) do |svx|
      x, y = 0, 0
      vx, vy = svx, svy

      until x > target.xmax || y < target.ymin
        x += vx
        y += vy
        vx -= 1 if vx > 0
        vy -= 1

        if target.includes?(x, y)
          count += 1
          break
        end
      end
    end
  end

  count
end

puts part1(target)
puts part2(target)
