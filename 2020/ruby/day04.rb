require 'set'

REQUIRED = %w[
  byr
  iyr
  eyr
  hgt
  hcl
  ecl
  pid
].to_set

def part1(passports)
  passports.count do |p|
    REQUIRED <= p.keys.to_set
  end
end

def part2(passports)
  passports.count do |p|
    REQUIRED <= p.keys.to_set &&
      p.all? { |k, v| valid?(k, v) }
  end
end

def valid?(field, value)
  case field
  when 'byr'
    m = value.match(/^(\d{4})$/)
    m && m[1].to_i >= 1920 && m[1].to_i <= 2002
  when 'iyr'
    m = value.match(/^(\d{4})$/)
    m && m[1].to_i >= 2010 && m[1].to_i <= 2020
  when 'eyr'
    m = value.match(/^(\d{4})$/)
    m && m[1].to_i >= 2020 && m[1].to_i <= 2030
  when 'hgt'
    m = value.match(/^(\d+)(cm|in)$/)
    m && if m[2] == 'cm'
      m[1].to_i >= 150 && m[1].to_i <= 193
    else
      m[1].to_i >= 59 && m[1].to_i <= 76
    end
  when 'hcl'
    value.match(/^#[0-9a-f]{6}$/)
  when 'ecl'
    %w(amb blu brn gry grn hzl oth).include?(value)
  when 'pid'
    value.match(/^\d{9}$/)
  else
    true
  end
end

passports = File.read("../in/day04.in").split("\n\n").map do |passport|
  passport.split(/\s/).map do |entry|
    entry.split(":")
  end.to_h
end

puts part1(passports)
puts part2(passports)
