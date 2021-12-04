data = <<~DATA
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
DATA

data = File.read('../in/day04.in')

drawn = data.lines.first.split(',').map(&:to_i)
cards = data.split("\n\n", 2)[1].split("\n\n").map do |card|
  card.lines.map { |l| l.split.map(&:to_i) }
end

def unmarked(c, cards, rows)
  cards[c].each.with_index.reduce(0) do |sum, (row, y)|
    sum + (row - rows[[c, y]]).sum
  end
end

locations = Array.new(100) { [] }
cards.each.with_index do |_, c|
  5.times do |y|
    5.times do |x|
      n = cards[c][y][x]
      locations[n] << [c, y, x]
    end
  end
end

rows = Hash.new { |h, k| h[k] = [] }
cols = Hash.new { |h, k| h[k] = [] }
winners = {}

drawn.each do |n|
  locations[n].each do |c, y, x|
    row = rows[[c, y]]
    col = cols[[c, x]]

    row << n
    col << n

    winners[c] = n * unmarked(c, cards, rows) if !winners.has_key?(c) && (row.length == 5 || col.length == 5)
  end
end

puts winners.to_a.first[1]
puts winners.to_a.last[1]
