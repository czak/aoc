require 'set'

ruledata = File.read('../in/day19.in').split("\n\n").first
messages = File.read('../in/day19-invalid.in').lines.map do |msg|
  msg.strip.tr('ab', '01').split('').each_slice(8).to_a.map { |b| b.join.to_i(2) }
end

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

va = flatten(42, rules).map { |a| a.flatten.join.tr('ab', '01').to_i(2) }.to_set
vb = flatten(31, rules).map { |a| a.flatten.join.tr('ab', '01').to_i(2) }.to_set

# 0: 8 11
# 8: a | a 8
# 11: a b | a 11 b

# 3-12 bytes

# 3
# aab
#
# 4
# aaab
#
# 5
# aaaab
# aaabb
#
# 6
# aaaaab
# aaaabb
#
# 7
# aaaaaab
# aaaaabb
# aaaabbb
#
# 8
# aaaaaaab
# aaaaaabb
# aaaaabbb
#
# 9
# aaaaaaaab
# aaaaaaabb
# aaaaaabbb
# aaaaabbbb
#
# 10
# aaaaaaaaab
# aaaaaaaabb
# aaaaaaabbb
# aaaaaabbbb
#
# 11
# aaaaaaaaaab
# aaaaaaaaabb
# aaaaaaaabbb
# aaaaaaabbbb
# aaaaaabbbbb
#
# 12
# aaaaaaaaaaab
# aaaaaaaaaabb
# aaaaaaaaabbb
# aaaaaaaabbbb
# aaaaaaabbbbb

res = messages.count do |m|
  case m.length
  when 3
    va.include?(m[0]) && va.include?(m[1]) && vb.include?(m[2])
  when 4
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && vb.include?(m[3])
  when 5
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && vb.include?(m[4]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && vb.include?(m[3]) && vb.include?(m[4])
  when 6
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && vb.include?(m[5]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && vb.include?(m[4]) && vb.include?(m[5])
  when 7
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && vb.include?(m[6]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && vb.include?(m[5]) && vb.include?(m[6]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && vb.include?(m[4]) && vb.include?(m[5]) && vb.include?(m[6])
  when 8
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && vb.include?(m[7]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && vb.include?(m[6]) && vb.include?(m[7]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && vb.include?(m[5]) && vb.include?(m[6]) && vb.include?(m[7])
  when 9
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && vb.include?(m[8]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && vb.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && vb.include?(m[5]) && vb.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8])
  when 10
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && va.include?(m[8]) && vb.include?(m[9]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && vb.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9])
  when 11
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && va.include?(m[8]) && va.include?(m[9]) && vb.include?(m[10]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && va.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && vb.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10])
  when 12
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && va.include?(m[8]) && va.include?(m[9]) && va.include?(m[10]) && vb.include?(m[11]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && va.include?(m[8]) && va.include?(m[9]) && vb.include?(m[10]) && vb.include?(m[11]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && va.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10]) && vb.include?(m[11]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && va.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10]) && vb.include?(m[11]) ||
    va.include?(m[0]) && va.include?(m[1]) && va.include?(m[2]) && va.include?(m[3]) && va.include?(m[4]) && va.include?(m[5]) && va.include?(m[6]) && vb.include?(m[7]) && vb.include?(m[8]) && vb.include?(m[9]) && vb.include?(m[10]) && vb.include?(m[11])
  else
    false
  end
end

# 126 is result from part 1 (covering all rules but the changed 8 and 11)
puts 126 + res
