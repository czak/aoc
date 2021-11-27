require 'scanf'

data = File.read('../in/day14.in')

# data = <<~DATA
# mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
# mem[8] = 11
# mem[7] = 101
# mem[8] = 0
# DATA

# data = <<~DATA
# mask = 000000000000000000000000000000X1001X
# mem[42] = 100
# mask = 00000000000000000000000000000000X0XX
# mem[26] = 1
# DATA

Mask = Struct.new(:value)
Mem = Struct.new(:address, :value)

program = data.lines.map do |line|
  if line.start_with?('mask')
    value = line.scanf('mask = %s').first
    Mask.new(value)
  else
    address, value = line.scanf('mem[%d] = %d')
    Mem.new(address, value)
  end
end

def part1(program)
  mem = {}
  ormask, andmask = nil

  program.each do |op|
    case op
    when Mask
      ormask = op.value.tr('X', '0').to_i(2)
      andmask = op.value.tr('X', '1').to_i(2)
    when Mem
      mem[op.address] =
        (op.value & andmask) | ormask
    end
  end

  mem.values.sum
end

def variations(address, mask)
  base = address | mask.tr('X', '0').to_i(2)

  # pozycje bitów na placeholdery od najmłodszego
  placeholders = mask.reverse.split('').map.with_index
    .select { |bit, pos| bit == 'X' }
    .map { |bit, pos| pos }

  def build(n, placeholders)
    ormask  = 0
    andmask = 2 ** 36 - 1

    placeholders.each.with_index do |shift, i|
      bit = (n & 2**i) >> i
      if bit == 1
        ormask |= 2**shift
      else
        andmask ^= 2**shift
      end
    end

    [ormask, andmask]
  end

  0.upto(2 ** placeholders.length - 1) do |n|
    ormask, andmask = build(n, placeholders)
    yield (base & andmask) | ormask
  end
end

def part2(program)
  mem = {}
  mask = nil

  program.each do |op|
    case op
    when Mask
      mask = op
    when Mem
      variations(op.address, mask.value) do |addr|
        mem[addr] = op.value
      end
    end
  end

  mem.values.sum
end


puts part1(program)
puts part2(program)
