def parse1(line)
  tokens = line.tr(' ', '').split('')

  out = []
  ops = []

  tokens.each do |t|
    case t
    when /[0-9]/
      out.push(t)
    when /[*+]/
      out.push(ops.pop) if ops.last =~ /[*+]/
      ops.push(t)
    when '('
      ops.push(t)
    when ')'
      out.push(ops.pop) until ops.last == '('
      ops.pop
    end
  end
  out.push(ops.pop)

  out
end

def parse2(line)
  tokens = line.tr(' ', '').split('')

  out = []
  ops = []

  tokens.each do |t|
    case t
    when /[0-9]/
      out.push(t)
    when '+'
      out.push(ops.pop) while ops.last =~ /[+]/
      ops.push(t)
    when '*'
      out.push(ops.pop) while ops.last =~ /[+*]/
      ops.push(t)
    when '('
      ops.push(t)
    when ')'
      out.push(ops.pop) until ops.last == '('
      ops.pop
    end
  end
  out.push(ops.pop) until ops.empty?

  out
end

def compute(tokens)
  stack = []
  tokens.each do |t|
    case t
    when /[0-9]/
      stack.push(t.to_i)
    when '+'
      stack.push(stack.pop + stack.pop)
    when '*'
      stack.push(stack.pop * stack.pop)
    end
  end
  raise 'bad compute' if stack.length != 1
  stack.first
end

def part1(data)
  data.lines.sum do |line|
    compute(parse1(line))
  end
end

def part2(data)
  data.lines.sum do |line|
    compute(parse2(line))
  end
end

data = File.read('../in/day18.in')

puts part1(data)
puts part2(data)
