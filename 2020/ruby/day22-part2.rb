data = <<~DATA
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
DATA

# data = File.read('../in/day22.in')

p1, p2 = data.split("\n\n").map do |block|
  block.lines[1..].map { |l| l.to_i }
end
