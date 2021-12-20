require 'set'

data = <<~DATA
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
DATA

data = File.read('../in/day20.in')

def parse(data)
  lines = data.lines(chomp: true)

  algorithm = lines[0].chars.map { |c| c == '#' }
  pixels = Hash.new(false)

  lines[2..].each_with_index do |line, y|
    line.chars.each_with_index do |c, x|
      pixels[[x, y]] = c == '#'
    end
  end

  [algorithm, pixels]
end

def enhance!(pixels, algorithm)
  res = if pixels.default
          Hash.new(algorithm[0b111111111])
        else
          Hash.new(algorithm[0b000000000])
        end

  xmin, xmax = pixels.keys.map(&:first).minmax
  ymin, ymax = pixels.keys.map(&:last).minmax

  (ymin-1).upto(ymax+1) do |y|
    (xmin-1).upto(xmax+1) do |x|
      offset = [
        [x-1, y-1], [x, y-1], [x+1, y-1],
        [x-1, y],   [x, y],   [x+1, y],
        [x-1, y+1], [x, y+1], [x+1, y+1],
      ].map { pixels[_1] ? 1 : 0 }.join.to_i(2)

      res[[x, y]] = algorithm[offset]
    end
  end

  res
end

algorithm, pixels = parse(data)

# part 1
2.times { pixels = enhance!(pixels, algorithm) }
puts pixels.count(&:last)

# part 2
48.times { pixels = enhance!(pixels, algorithm) }
puts pixels.count(&:last)
