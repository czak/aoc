data = File.read('../in/day25.in')

# data = <<~DATA
# 5764801
# 17807724
# DATA

card_pubkey, door_pubkey = data.lines.map { |l| l.strip.to_i }

def find_loop_size(subject, target)
  value = 1
  for i in 1..10000000
    value *= subject
    value %= 20201227 

    return i if value == target
  end
  nil
end

def transform(subject, loop_size)
  value = 1
  for i in 1..loop_size
    value *= subject
    value %= 20201227 
  end
  value
end

card_loop_size = find_loop_size(7, card_pubkey)
door_loop_size = find_loop_size(7, door_pubkey)

puts transform(door_pubkey, card_loop_size)
puts transform(card_pubkey, door_loop_size)
