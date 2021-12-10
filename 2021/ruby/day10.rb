data = <<~DATA
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
DATA

data = File.read('../in/day10.in')

lines = data.lines(chomp: true)

MAP = {
  '(' => ')',
  '[' => ']',
  '{' => '}',
  '<' => '>',
}

def parse(line)
  stack = []
  line.each_char do |c|
    if MAP.keys.include?(c)
      stack.push(MAP[c])
    elsif stack.last == c
      stack.pop
    else
      return [nil, c]
    end
  end
  [stack, nil]
end

def part1(lines)
  lines.map { parse(_1).last }.compact.sum do |c|
    case c
    when ')' then 3
    when ']' then 57
    when '}' then 1197
    when '>' then 25137
    end
  end
end

def part2(lines)
  scores = lines.map do |l|
    stack, _ = parse(l)
    next if stack.nil?

    stack.reverse.reduce(0) do |sum, c|
      sum *= 5
      sum += case c
             when ')' then 1
             when ']' then 2
             when '}' then 3
             when '>' then 4
             end
    end
  end.compact

  scores.sort[scores.length/2]
end

puts part1(lines)
puts part2(lines)
