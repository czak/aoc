data = <<~EXAMPLE
A Y
B X
C Z
EXAMPLE

data = $stdin.read

SCORE_MAP = {
  "A X" => 1 + 3,
  "B X" => 1 + 0,
  "C X" => 1 + 6,
  "A Y" => 2 + 6,
  "B Y" => 2 + 3,
  "C Y" => 2 + 0,
  "A Z" => 3 + 0,
  "B Z" => 3 + 6,
  "C Z" => 3 + 3,
}

SELECT_MAP = {
  "A X" => "A Z",
  "B X" => "B X",
  "C X" => "C Y",
  "A Y" => "A X",
  "B Y" => "B Y",
  "C Y" => "C Z",
  "A Z" => "A Y",
  "B Z" => "B Z",
  "C Z" => "C X",
}

puts data.lines(chomp: true).sum { |l| SCORE_MAP[l] }
puts data.lines(chomp: true).sum { |l| SCORE_MAP[SELECT_MAP[l]] }
