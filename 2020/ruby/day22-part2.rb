require 'set'

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

data = <<~DATA
Player 1:
43
19

Player 2:
2
29
14
DATA

data = File.read('../in/day22.in')

p1, p2 = data.split("\n\n").map do |block|
  block.lines[1..].map { |l| l.to_i }
end

def play(p1, p2, level: 1)
  seen = Set.new

  round = 1
  until p1.empty? || p2.empty?
    if seen.include?([p1, p2])
      # puts "Exiting Game #{level}"
      return 1
    end

    seen.add([p1, p2])

    # puts "--- Round #{round} (Game #{level}) ---"
    # puts "P1: #{p1.inspect}"
    # puts "P2: #{p2.inspect}"

    c1 = p1.shift
    c2 = p2.shift

    # puts "P1 plays: #{c1}"
    # puts "P2 plays: #{c2}"
    # puts

    result = if p1.length >= c1 && p2.length >= c2
               play(p1[0...c1], p2[0...c2], level: level+1)
             elsif c1 > c2
               1
             else
               2
             end

    if result == 1
      p1.push(c1, c2)
    else
      p2.push(c2, c1)
    end

    round += 1
  end

  p2.empty? ? 1 : 2
end

winner = play(p1, p2, level: 1) == 1 ? p1 : p2

puts "== POST =="
puts "Winner: #{winner}"
puts p1.inspect
puts p2.inspect

res = winner.map.with_index.reduce(0) do |sum, (card, i)|
  sum + card * (winner.length - i)
end

puts res
