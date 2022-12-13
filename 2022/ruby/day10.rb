require 'scanf'

data = <<~EXAMPLE
noop
addx 3
addx -5
EXAMPLE

data = $stdin.read

TICK_COUNTS = {
  "noop" => 1,
  "addx" => 2,
}

instructions = data.scanf("%s %d") { _1.itself }

def simulate(instructions)
  return to_enum(:simulate, instructions) unless block_given?

  i = instructions.each
  x = 1

  cycle = 1
  ticks = 0

  sum = 0

  loop do
    opcode, value = i.peek

    yield(cycle, x)

    ticks = TICK_COUNTS[opcode] if ticks == 0
    ticks -= 1

    if ticks == 0
      x += value if opcode == "addx"
      i.next
    end

    cycle += 1
  end

  sum
end

puts "Part 1:", simulate(instructions).sum { |cycle, x| (cycle - 20) % 40 == 0 ? cycle * x : 0 }

puts "Part 2:"
simulate(instructions) do |cycle, x|
  beam = (cycle - 1) % 40

  if (x - beam).abs <= 1
    print '#'
  else
    print ' '
  end

  puts if beam == 39
end
