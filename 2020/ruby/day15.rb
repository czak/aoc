data = '6,3,15,13,1,0'.split(',').map(&:to_i)
# data = '3,1,2'.split(',').map(&:to_i)

class TwoQueue
  def initialize
    @a = []
  end

  def push(num)
    @a.shift if @a.length == 2
    @a.push(num)
  end

  def first_time?
    @a.length == 1
  end

  def diff
    @a[1] - @a[0]
  end
end

def part1(data)
  mem = Hash.new { |h, k| h[k] = TwoQueue.new }

  data.each.with_index do |n, i|
    mem[n].push(i+1)
  end

  n = data.last

  (data.length+1).upto(30000000) do |turn|
    last = mem[n]
    if last.first_time?
      n = 0
    else
      n = last.diff
    end
    mem[n].push(turn)

    puts "#{turn}: #{n}"
  end
end

part1(data)
