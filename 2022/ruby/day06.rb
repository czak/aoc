data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
data = "bvwbjplbgvbhsrlpgdmjqwftvncz"
data = "nppdvjthqldpwncqszvftbrmjlhg"
data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"

data = $stdin.read

def start_of_packet(data, len)
  data.chars.each_cons(len).find_index { |seq| seq.uniq.length == len } + len
end

puts start_of_packet(data, 4)
puts start_of_packet(data, 14)
