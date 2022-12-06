require 'scanf'

data = <<~SIMPLE
on x=10..13,y=10..13,z=10..13
on x=11..12,y=11..12,z=11..12
SIMPLE

# data = <<~EXAMPLE
# on x=10..12,y=10..12,z=10..12
# on x=11..13,y=11..13,z=11..13
# off x=9..11,y=9..11,z=9..11
# on x=10..10,y=10..10,z=10..10
# EXAMPLE

# data = <<~EXAMPLE
# on x=-20..26,y=-36..17,z=-47..7
# on x=-20..33,y=-21..23,z=-26..28
# on x=-22..28,y=-29..23,z=-38..16
# on x=-46..7,y=-6..46,z=-50..-1
# on x=-49..1,y=-3..46,z=-24..28
# on x=2..47,y=-22..22,z=-23..27
# on x=-27..23,y=-28..26,z=-21..29
# on x=-39..5,y=-6..47,z=-3..44
# on x=-30..21,y=-8..43,z=-13..34
# on x=-22..26,y=-27..20,z=-29..19
# off x=-48..-32,y=26..41,z=-47..-37
# on x=-12..35,y=6..50,z=-50..-2
# off x=-48..-32,y=-32..-16,z=-15..-5
# on x=-18..26,y=-33..15,z=-7..46
# off x=-40..-22,y=-38..-28,z=23..41
# on x=-16..35,y=-41..10,z=-47..6
# off x=-32..-23,y=11..30,z=-14..3
# on x=-49..-5,y=-3..45,z=-29..18
# off x=18..30,y=-20..-8,z=-3..13
# on x=-41..9,y=-7..43,z=-33..15
# EXAMPLE

# data = $stdin.read

Cuboid = Struct.new(:xrange, :yrange, :zrange) do
  def include?(x, y, z)
    xrange.include?(x) && yrange.include?(y) && zrange.include?(z)
  end

  def origin
    [xrange.begin, yrange.begin, zrange.begin]
  end

  def size
    xrange.size * yrange.size * zrange.size
  end
end

instructions = data.scanf("%s x=%d..%d,y=%d..%d,z=%d..%d\n") do |toggle, xmin, xmax, ymin, ymax, zmin, zmax|
  [
    toggle,
    Cuboid.new(xmin...xmax+1, ymin...ymax+1, zmin...zmax+1),
  ]
end

def partition(c1, c2)
  xs = [c1.xrange.begin, c1.xrange.end, c2.xrange.begin, c2.xrange.end].sort
  ys = [c1.yrange.begin, c1.yrange.end, c2.yrange.begin, c2.yrange.end].sort
  zs = [c1.zrange.begin, c1.zrange.end, c2.zrange.begin, c2.zrange.end].sort

  xranges = xs.each_cons(2).filter { |b, e| b != e }.map { |b, e| b...e }
  yranges = ys.each_cons(2).filter { |b, e| b != e }.map { |b, e| b...e }
  zranges = zs.each_cons(2).filter { |b, e| b != e }.map { |b, e| b...e }

  xranges.product(yranges, zranges).map { |ranges| Cuboid.new(*ranges) }
end

def union(c1, c2)
  partition(c1, c2).select do |cuboid|
    c1.include?(*cuboid.origin) || c2.include?(*cuboid.origin)
  end
end

def subtract(c1, c2)
  partition(c1, c2).select do |cuboid|
    c1.include?(*cuboid.origin) && !c2.include?(*cuboid.origin)
  end
end

# c1 = Cuboid.new(10...13, 10...13, 10...13)
# c2 = Cuboid.new(11...14, 11...14, 11...14)
# c3 = Cuboid.new(9...12, 9...12, 9...12)

system = [instructions[0].last]

instructions[1..].each.with_index do |(toggle, c2), i|
  system = system.map do |c1|
    if toggle == "on"
      union(c1, c2)
    else
      subtract(c1, c2)
    end
  end.flatten.uniq

  pp system
end

puts system.map(&:size).sum

# instructions.each do |

# system = [c1]
#
# system = system.map do |cuboid|
#   union(cuboid, c2)
# end.flatten
#
# puts system.map(&:size).sum
#
# system = system.map do |cuboid|
#   subtract(cuboid, c3)
# end
#
# puts system.map(&:size).sum
#
# system = system.map do |cuboid|
#   subtract(cuboid, c3)
# end
#
# # pp union(c1, c2).map(&:size).sum
