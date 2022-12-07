Node = Struct.new(:size, :children, :parent)

data = <<~EXAMPLE
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
EXAMPLE

data = $stdin.read

cwd = root = Node.new(0, {}, nil)

data.lines(chomp: true).each do |line|
  case line
  when /\$ cd \//
    cwd = root
  when /\$ cd \.\./
    cwd = cwd.parent
  when /\$ cd (.+)/
    cwd = cwd.children[$1]
  when /dir (.+)/
    cwd.children[$1] = Node.new(0, {}, cwd)
  when /(\d+) (.+)/
    cwd.children[$2] = Node.new($1.to_i, {}, cwd)
  end
end

def precalc(node)
  node.size += node.children.sum { |name, child| precalc(child) }
end

def lsdirs(node, found = [])
  if node.children.any?
    found << node
    node.children.each { |name, child| lsdirs(child, found) }
  end
  found
end

def part1(root)
  lsdirs(root).select { |dir| dir.size <= 100_000 }.sum(&:size)
end

def part2(root)
  required = 30_000_000 - (70_000_000 - root.size)
  lsdirs(root).select { |dir| dir.size >= required }.min_by(&:size).size
end

precalc(root)

puts part1(root)
puts part2(root)
