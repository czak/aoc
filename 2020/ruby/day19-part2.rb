require 'set'

data = File.read('../in/day19-part2.in')
ruledata = data.split("\n\n").first
messages = data.split("\n\n").last.lines.map(&:strip)

# IN:
# 0: 1 2
# 1: "a"
# 2: 1 3 | 3 1
# 3: "b"
#
# OUT:
# {
#   "0" => [["1", "2"]],
#   "1" => "a",
#   "2" => [["1", "3"], ["3", "1"]],
#   "3" => "b"
# }
def parse(ruledata)
  ruledata.lines.map do |line|
    id, val = line.split(': ')
    if m = val.match(/"(.)"/)
      [id.to_i, m[1]]
    else
      [id.to_i, val.split(' | ').map { |a| a.split.map(&:to_i) }]
    end
  end.to_h
end

# ruledata = <<~DATA
# 0: 1 2
# 1: "a"
# 2: 1 3 | 3 1
# 3: "b"
# DATA
#
# ruledata = <<~DATA
# 0: 4 1 5
# 1: 2 3 | 3 2
# 2: 4 4 | 5 5
# 3: 4 5 | 5 4
# 4: "a"
# 5: "b"
# DATA

def flatten(id, rules)
  rule = rules[id]
  if rule.kind_of?(String)
    [rule]
  else
    rule.reduce([]) do |sum, subrule|
      sum + subrule.map { |s| flatten(s, rules) }
        .reduce { |prod, s| prod.product(s) }
    end
  end
end

rules = parse(ruledata)

valid = flatten(0, rules).map { |a| a.flatten.join }.to_set
valid42 = flatten(42, rules).map { |a| a.flatten.join }.to_set
valid31 = flatten(31, rules).map { |a| a.flatten.join }.to_set

puts messages.count { |msg| valid.include?(msg) }

# messages repeating any part in 42
