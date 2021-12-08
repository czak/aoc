require 'set'

data = <<~DATA
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
DATA

data = File.read('../in/day08.in')

def readout(digits, output)
  d1 = digits.find { |d| d.size == 2 }
  d4 = digits.find { |d| d.size == 4 }
  d7 = digits.find { |d| d.size == 3 }
  d8 = digits.find { |d| d.size == 7 }

  #  aaaa
  # b    c
  # b    c
  #  dddd
  # .    f
  # .    f
  #  gggg
  d9 = digits.find do |d|
    d.size == 6 && (d4 & d).size == 4
  end

  #  aaaa 
  # b    c
  # b    c
  #  .... 
  # e    f
  # e    f
  #  gggg 
  d0 = digits.find do |d|
    d.size == 6 && (d1 & d).size == 2 && d != d9
  end

  #  aaaa 
  # .    c  
  # .    c
  #  dddd 
  # .    f
  # .    f
  #  gggg 
  d3 = digits.find do |d|
    d.size == 5 && (d9 & d).size == 5 && (d1 & d).size == 2
  end

  #  aaaa 
  # b    .
  # b    .
  #  dddd 
  # .    f
  # .    f
  #  gggg 
  d5 = digits.find do |d|
    d.size == 5 && (d9 & d).size == 5 && (d1 & d).size == 1
  end

  #  aaaa 
  # b    .
  # b    .
  #  dddd 
  # e    f
  # e    f
  #  gggg 
  d6 = digits.find do |d|
    d.size == 6 && (d5 & d).size == 5 && (d1 & d).size == 1
  end

  #  aaaa 
  # .    c
  # .    c
  #  dddd 
  # e    .
  # e    .
  #  gggg 
  d2 = digits.find do |d|
    d.size == 5 && (d9 & d).size == 4
  end

  map = {
    d0 => 0,
    d1 => 1,
    d2 => 2,
    d3 => 3,
    d4 => 4,
    d5 => 5,
    d6 => 6,
    d7 => 7,
    d8 => 8,
    d9 => 9,
  }

  output.map { map[_1] }
end

displays = data.lines.map do |line|
  digits, output = line.split(' | ')

  readout(
    digits.split.map { _1.split('').to_set }.to_set,
    output.split.map { _1.split('').to_set }
  )
end

puts displays.flatten.count { |n| [1, 4, 7, 8].include?(n) }
puts displays.map { |d| d.join.to_i }.sum
