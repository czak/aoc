Node = Struct.new(:value, :next, :prev)

def make_list(s)
  cups = s.split('').map(&:to_i)

  head = nil
  tail = nil

  head = tail = Node.new(1_000_000, nil, nil)

  999_999.downto(10).each do |n|
    head = Node.new(n, head, nil)
  end

  cups.reverse.each do |n|
    head = Node.new(n, head, nil)
  end

  tail.next = head

  # set prev
  prev = head
  current = head.next
  while current.prev == nil
    current.prev = prev
    prev = current
    current = current.next
  end

  head
end

# current = make_list('389125467') # example
current = make_list('853192647')

def debug(head, label:, count:, mark_head:)
  print "#{label}: "
  for i in 0...count
    if i == 0 && mark_head
      print "(#{head.value}) "
    else
      print "#{head.value} "
    end
    head = head.next
  end
  puts
end

def find_destination(current, pa, pb, pc)
  d = current
  loop do
    d -= 1
    d = 1_000_000 if d == 0
    next if d == pa || d == pb || d == pc
    return d
  end
end

for move in 1..10_000_000
  if move % 100_000 == 0
    puts "--- move #{move} ---"
  end

  # debug(current, label: "cups", count: 9, mark_head: true)

  # keep track of picked 3 cups
  picked = current.next

  # debug(picked, label: "pick up", count: 3, mark_head: false)

  # remove picked from list
  current.next = current.next.next.next.next

  d = find_destination(current.value, picked.value, picked.next.value, picked.next.next.value)
  # puts "destination: #{d}"

  destination = current
  until destination.value == d
    destination = destination.prev
  end

  after = destination.next
  destination.next = picked
  picked.next.next.next = after

  current = current.next

  # puts
end

def part2(current)
  head = current
  until head.value == 1
    head = head.next
  end

  head.next.value * head.next.next.value
end

puts part2(current)
