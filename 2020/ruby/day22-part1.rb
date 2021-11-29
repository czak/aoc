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

data = File.read('../in/day22.in')

p1, p2 = data.split("\n\n").map do |block|
  block.lines[1..].map { |l| l.to_i }
end

until p1.empty? || p2.empty?
  c1 = p1.shift
  c2 = p2.shift

  if c1 > c2
    p1.push(c1, c2)
  else
    p2.push(c2, c1)
  end
end

# Part 1
winner = p2.empty? ? p1 : p2

res = winner.map.with_index.reduce(0) do |sum, (card, i)|
  sum + card * (winner.length - i)
end

puts res

