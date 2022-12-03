data = <<~EXAMPLE
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
EXAMPLE

data = $stdin.read

puts "Part 1"
puts data.lines(chomp: true)
  .map(&:chars).transpose.map { |a| a.tally.max_by { |k, v| v } }
  .map(&:first)
  .join

puts "Part 2"
puts data.lines(chomp: true)
  .map(&:chars).transpose.map { |a| a.tally.min_by { |k, v| v } }
  .map(&:first)
  .join
