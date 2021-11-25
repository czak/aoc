require 'set'

Instruction = Struct.new(:operation, :argument)

class Computer
  attr_reader :accumulator, :offset

  def initialize(program)
    @program = program
    @accumulator = 0
    @offset = 0
  end

  def step
    instruction = @program[offset]

    case instruction.operation
    when 'acc'
      @accumulator += instruction.argument
      @offset += 1
    when 'jmp'
      @offset += instruction.argument
    when 'nop'
      @offset += 1
    end
  end
end

def parse(code)
  code.lines.map do |line|
    Instruction.new(line[0...3], line[4..].to_i)
  end
end

def run_to_end_or_loop(program)
  Computer.new(program).tap do |computer|
    visited = Set.new([0])
    loop do
      computer.step
      break if computer.offset == program.length || visited.include?(computer.offset)
      visited << computer.offset
    end
  end
end

def part1(program)
  run_to_end_or_loop(program).accumulator
end

def part2(program)
  (0...program.length).each do |offset|
    instruction = program[offset]

    case instruction.operation
    when 'jmp'
      program[offset] = Instruction.new('nop', instruction.argument)
    when 'nop'
      program[offset] = Instruction.new('jmp', instruction.argument)
    else
      next
    end

    cpu = run_to_end_or_loop(program)
    return cpu.accumulator if cpu.offset == program.length

    program[offset] = instruction
  end
end


program = parse(File.read('../in/day08.in'))

puts part1(program)
puts part2(program)
