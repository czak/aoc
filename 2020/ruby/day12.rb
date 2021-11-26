def part1(data)
  x, y = 0, 0
  angle = 0

  data.each do |row|
    action = row[0]
    value = row[1..].to_i

    case action
    when 'N'
      y += value
    when 'S'
      y -= value
    when 'E'
      x += value
    when 'W'
      x -= value
    when 'L'
      angle = (angle + value) % 360
    when 'R'
      angle = (angle - value) % 360
    when 'F'
      case angle
      when 0 then x += value
      when 90 then y += value
      when 180 then x -= value
      when 270 then y -= value
      else raise "Bad #{angle}"
      end
    end
  end

  x.abs + y.abs
end

def part2(data)
  x, y = 0, 0
  wx, wy = 10, 1

  data.each do |row|
    action = row[0]
    value = row[1..].to_i

    case action
    when 'N'
      wy += value
    when 'S'
      wy -= value
    when 'E'
      wx += value
    when 'W'
      wx -= value
    when 'L'
      case value
      when 90 then wx, wy = -wy, wx
      when 180 then wx, wy = -wx, -wy
      when 270 then wx, wy = wy, -wx
      end
    when 'R'
      case value
      when 90 then wx, wy = wy, -wx
      when 180 then wx, wy = -wx, -wy
      when 270 then wx, wy = -wy, wx
      end
    when 'F'
      x += value * wx
      y += value * wy
    end
  end

  x.abs + y.abs
end

data = <<~DATA.lines.map(&:strip)
F10
N3
F7
R90
F11
DATA

data = File.readlines('../in/day12.in').map(&:strip)

puts part1(data)
puts part2(data)
