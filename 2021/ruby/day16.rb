require 'stringio'

# data = 'D2FE28'
# data = '9C0141080250320F1802104A08'
data = File.read('../in/day16.in')

bits = data.each_char.map { |hex| hex.to_i(16).to_s(2).rjust(4, '0') }.join

Packet = Struct.new(:version, :type, :value, :children)

def parse(io)
  packet = Packet.new(nil, nil, 0, [])

  state = :header
  children_done = nil

  loop do
    case state
    when :header
      packet.version = io.read(3).to_i(2)
      packet.type = io.read(3).to_i(2)
      if packet.type == 4
        state = :literal
      else
        state = :operator
      end

    when :literal
      flag = io.read(1).to_i(2)
      packet.value = packet.value << 4 | io.read(4).to_i(2)
      if flag == 0
        state = :end
      end

    when :operator
      length_type = io.read(1).to_i(2)
      if length_type == 0
        count = io.read(15).to_i(2)
        offset = io.pos + count
        children_done = -> { io.pos == offset }
      else
        count = io.read(11).to_i(2)
        children_done = -> { packet.children.count == count }
      end
      state = :child

    when :child
      if children_done.call
        state = :end
      else
        packet.children << parse(io)
      end

    when :end
      return packet
    end
  end
end

def versions(packet)
  packet.version + packet.children.sum { |child| versions(child) }
end

def evaluate(packet)
  return packet.value if packet.type == 4

  values = packet.children.map { evaluate(_1) }

  case packet.type
  when 0 then values.sum
  when 1 then values.reduce(:*)
  when 2 then values.min
  when 3 then values.max
  when 5 then values[0] > values[1] ? 1 : 0
  when 6 then values[0] < values[1] ? 1 : 0
  when 7 then values[0] == values[1] ? 1 : 0
  end
end

packet = parse(StringIO.new(bits))

puts versions(packet)
puts evaluate(packet)
