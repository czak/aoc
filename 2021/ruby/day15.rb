require 'set'
require 'pqueue'

data = <<~DATA
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
DATA

data = File.read('../in/day15.in')

grid = data.lines(chomp: true).map { |l| l.chars.map(&:to_i) }

def solve(grid, mul:)
  size = grid.size

  # Build graph
  graph = Hash.new { |h, k| h[k] = {} }

  for y in 0...size*mul
    for x in 0...size*mul
      current = [x, y]
      graph[current][[x-1, y]] = (grid[y % size][(x-1) % size] + (x - 1) / size + y / size - 1) % 9 + 1 if x > 0
      graph[current][[x, y-1]] = (grid[(y-1) % size][x % size] + x / size + (y - 1) / size - 1) % 9 + 1 if y > 0
      graph[current][[x+1, y]] = (grid[y % size][(x+1) % size] + (x + 1) / size + y / size - 1) % 9 + 1 if x < size*mul - 1
      graph[current][[x, y+1]] = (grid[(y+1) % size][x % size] + x / size + (y + 1) / size - 1) % 9 + 1 if y < size*mul - 1
    end
  end

  # Dijkstra
  dist = { [0, 0] => 0 }
  dist.default = 1_000_000

  q = PQueue.new([[0, 0]]) { |a, b| dist[b] <=> dist[a] }

  while u = q.pop
    graph[u].each do |v, cost|
      alt = dist[u] + cost
      if alt < dist[v]
        dist[v] = alt
        q.push(v)
      end
    end
  end

  dist[[size * mul - 1, size * mul - 1]]
end

puts solve(grid, mul: 1)
puts solve(grid, mul: 5)
