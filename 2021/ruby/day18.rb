data = <<~DATA
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
DATA

data = File.read('../in/day18.in')

Node = Struct.new(:val, :l, :r, :p, :dir)

def tree(number, p=nil, dir=nil)
  if number.is_a? Integer
    Node.new(number, nil, nil, p, dir)
  else
    Node.new(nil, nil, nil, p, dir).tap do |n|
      n.l = tree(number.first, n, :left)
      n.r = tree(number.last, n, :right)
    end
  end
end

def explode(root)
  def find(node, level=0)
    return nil if node.val
    return node if level == 4
    find(node.l, level+1) || find(node.r, level+1)
  end

  find(root)&.tap do |n|
    c = n
    c = c.p until c.nil? || c.dir == :right
    if c
      c = c.p
      c = c.l
      c = c.r until c.val
      c.val += n.l.val
    end

    c = n
    c = c.p until c.nil? || c.dir == :left
    if c
      c = c.p
      c = c.r
      c = c.l until c.val
      c.val += n.r.val
    end

    n.val = 0
  end
end

def split(root)
  def find(node)
    return node if node.val && node.val >= 10
    return nil if node.val
    find(node.l) || find(node.r)
  end

  find(root)&.tap do |n|
    n.l = Node.new((n.val.to_f / 2).floor, nil, nil, n, :left)
    n.r = Node.new((n.val.to_f / 2).ceil, nil, nil, n, :right)
    n.val = nil
  end
end

def reduce(t)
  loop do
    explode(t) || split(t) || break
  end
end

def magnitude(root)
  if root.val
    root.val
  else
    3 * magnitude(root.l) + 2 * magnitude(root.r)
  end
end

def sum(l, r)
  Node.new(nil, l, r).tap do |n|
    n.l.p = n
    n.l.dir = :left
    n.r.p = n
    n.r.dir = :right
    reduce(n)
  end
end

def part1(data)
  nodes = data.each_line.map { |l| tree(eval(l)) }
  magnitude(nodes.reduce(&method(:sum)))
end

def part2(data)
  data.lines
    .map { eval(_1) }
    .combination(2)
    .flat_map { [[_1, _2], [_2, _1]] }
    .map { magnitude(sum(tree(_1), tree(_2))) }
    .max
end

puts part1(data)
puts part2(data)
