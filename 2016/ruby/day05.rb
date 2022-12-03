require 'digest'

id = "ffykfhsq"

def md5(key)
end

def part1(id)
  password = ""
  md5 = Digest::MD5.new

  0.upto(10000000) do |index|
    md5.reset
    md5 << "#{id}#{index}"
    hash = md5.hexdigest

    if hash[0...5] == "00000"
      $stderr.puts "#{index}: #{hash}"

      password << hash[5]
      return password if password.length == 8
    end
  end
end

def part2(id)
  password = "________"

  0.upto(100000000) do |index|
    hash = Digest::MD5.hexdigest("#{id}#{index}")

    if hash[0...5] == "00000"
      $stderr.puts "#{index}: #{hash}"

      position = hash[5].to_i(16)
      if position <= 7 && password[position] == "_"
        password[position] = hash[6]
        $stderr.puts password
      else
        $stderr.puts "- skipping"
      end

      return password if password.chars.all? { |ch| ch != "_" }
    end
  end
end

puts "Part 1:", part1(id)
puts "Part 2:", part2(id)
