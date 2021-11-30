Node = Struct.new(:value, :next)

def make_list(s)
  cups = s.split('').map(&:to_i)

  head = nil
  tail = nil

  cups.reverse.each.with_index do |n, i|
    if i == 0
      head = tail = Node.new(n, nil)
    else
      head = Node.new(n, head)
    end
  end

  tail.next = head

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
    d = 9 if d == 0
    next if d == pa || d == pb || d == pc
    return d
  end
end

for move in 1..100
  puts "--- move #{move} ---"

  debug(current, label: "cups", count: 9, mark_head: true)

  # keep track of picked 3 cups
  picked = current.next

  debug(picked, label: "pick up", count: 3, mark_head: false)
  
  # remove picked from list
  current.next = current.next.next.next.next

  d = find_destination(current.value, picked.value, picked.next.value, picked.next.next.value)
  puts "destination: #{d}"

  destination = current
  until destination.value == d
    destination = destination.next
  end

  after = destination.next
  destination.next = picked
  picked.next.next.next = after

  current = current.next

  puts
end

def dump_res(current)
  head = current
  until head.value == 1
    head = head.next
  end

  for i in 0...8
    head = head.next
    print head.value
  end

  puts
end

dump_res(current)

# debug(current, label: "cups", count: 9, mark_head: true)
