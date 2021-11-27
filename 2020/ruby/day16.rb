require 'scanf'
require 'set'

data = File.read('../in/day16.in')

# data = <<~DATA
# class: 1-3 or 5-7
# row: 6-11 or 33-44
# seat: 13-40 or 45-50
#
# your ticket:
# 7,1,14
#
# nearby tickets:
# 7,3,47
# 40,4,50
# 55,2,20
# 38,6,12
# DATA

# data = <<~DATA
# class: 0-1 or 4-19
# row: 0-5 or 8-19
# seat: 0-13 or 16-19
#
# your ticket:
# 11,12,13
#
# nearby tickets:
# 3,9,18
# 15,1,5
# 5,14,9
# DATA

def parse(data)
  rules, your, nearby = data.split("\n\n")
  [
    rules.lines.map do |rule|
      field, a, b, c, d = rule.scanf('%[a-z ]: %d-%d or %d-%d')
      [field, (a..b).to_set | (c..d).to_set]
    end.to_h,
    your.lines[1].split(',').map(&:to_i),
    nearby.lines[1..].map do |l|
      l.split(',').map(&:to_i)
    end
  ]
end

def part1(rules, nearby)
  allowed = rules.values.reduce(:|)

  nearby.flatten.sum do |n|
    allowed.include?(n) ? 0 : n
  end
end

def part2(rules, your, nearby)
  allowed = rules.values.reduce(:|)

  valid = nearby.select do |t|
    t.all? { |n| allowed.include?(n) }
  end

  # final mapping rule => id
  dict = {}
  sets = valid.transpose.map(&:to_set)

  until rules.empty?
    predictions = rules.map do |name, ruleset|
      [
        name,
        sets.map.with_index { |s, i| [i, s <= ruleset] }.select { |s| s.last }.map { |s| s.first }
      ]
    end.to_h

    # Move certain predictions over to dict
    predictions
      .select { |name, p| p.length == 1 }
      .each do |name, p|
        dict[name] = p.first
        rules.delete(name)
        sets[p.first] = [-1].to_set
      end
  end

  fields = dict.select { |f, i| f.start_with?('departure') }

  fields.values.reduce(1) do |total, i|
    total * your[i]
  end
end

rules, your, nearby = parse(data)

puts part1(rules, nearby)
puts part2(rules, your, nearby)
