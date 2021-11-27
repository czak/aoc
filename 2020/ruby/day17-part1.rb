require 'set'

data = File.read('../in/day17.in')

# data = <<~DATA
# .#.
# ..#
# ###
# DATA

def parse(data)
  data.lines.map.with_index do |line, y|
    line.strip.split('').map.with_index.select do |cube, x|
      cube == '#'
    end.map { |cube, x| [x, y, 0] }
  end.flatten(1).to_set
end

SURROUNDINGS = [-1, 0, 1].product([-1, 0, 1], [-1, 0, 1])
  .to_set
  .delete([0, 0, 0])
  .freeze

def active_surroundings(coord, world)
  SURROUNDINGS.sum do |delta|
    c = [
      coord[0] + delta[0],
      coord[1] + delta[1],
      coord[2] + delta[2],
    ]
    world.include?(c) ? 1 : 0
  end
end

def simulate(world)
  xmin, xmax = world.minmax_by { |c| c[0] }
    .map { |c| c[0] }
  ymin, ymax = world.minmax_by { |c| c[1] }
    .map { |c| c[1] }
  zmin, zmax = world.minmax_by { |c| c[2] }
    .map { |c| c[2] }

  after = Set.new(world)

  ((xmin-1)..(xmax+1)).each do |x|
    ((ymin-1)..(ymax+1)).each do |y|
      ((zmin-1)..(zmax+1)).each do |z|
        c = [x, y, z]
        n = active_surroundings(c, world)

        if world.include?(c)
          # cube is active
          if n != 2 && n != 3
            # but not surounded by 2 or 3 active
            after.delete(c)
          end
        else
          # cube is inactive
          if n == 3
            # and surrounded by 3 active
            after.add(c)
          end
        end
      end
    end
  end

  after
end

def debug(world)
  xmin, xmax = world.minmax_by { |c| c[0] }
    .map { |c| c[0] }
  ymin, ymax = world.minmax_by { |c| c[1] }
    .map { |c| c[1] }
  zmin, zmax = world.minmax_by { |c| c[2] }
    .map { |c| c[2] }

  (zmin..zmax).each do |z|
    (ymin..ymax).each do |y|
      (xmin..xmax).each do |x|
        if world.include?([x, y, z])
          print '#'
        else
          print '.'
        end
      end
      puts
    end
    puts
  end
end

def part1(initial)
  world = Set.new(initial)

  6.times { world = simulate(world) }

  world.length
end

initial = parse(data)

puts part1(initial)
