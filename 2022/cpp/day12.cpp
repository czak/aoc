#include "stdafx.h"

istringstream example{R"(Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
)"};

using Point = pair<int, int>;
using Grid = vector<vector<int>>;
using Graph = vector<pair<int, int>>;

tuple<Grid, int, int> parse(istream& input)
{
  Grid grid{{}};
  Point start, end;
  int x = 0, y = 0, width = 0;

  char c;
  while (input.get(c)) {
    if (c == '\n') {
      input.peek();
      if (input.eof()) break;
      grid.push_back({});
      width = x;
      x = 0;
      y++;
    } else {
      if (c == 'S') {
        start = {x, y};
        grid.back().push_back(0);
      } else if (c == 'E') {
        end = {x, y};
        grid.back().push_back(25);
      } else {
        grid.back().push_back(c - 'a');
      }
      x++;
    }
  }

  return {grid, width * start.second + start.first, width * end.second + end.first};
}

tuple<Graph, int, int> make_graph(const Grid& grid)
{
  Graph g{};
  int width = grid[0].size();
  int height = grid.size();

  for (int y = 0; y < height; y++) {
    for (int x = 0; x < width; x++) {
      int l = width * y + x;
      int elev = grid[y][x];

      // clang-format off
      if (y > 0          && elev - grid[y - 1][x] <= 1) g.push_back({l, l - width});
      if (y < height - 1 && elev - grid[y + 1][x] <= 1) g.push_back({l, l + width});
      if (x > 0          && elev - grid[y][x - 1] <= 1) g.push_back({l, l - 1});
      if (x < width - 1  && elev - grid[y][x + 1] <= 1) g.push_back({l, l + 1});
      // clang-format on
    }
  }

  return {g, width, height};
}

vector<int> solve(const Graph& graph, int n, int start)
{
  vector<int> distance(n, 1000000);
  distance[start] = 0;

  for (int i = 0; i < n; i++) {
    for (auto [a, b] : graph) {
      distance[b] = min(distance[b], distance[a] + 1);
    }
  }

  return distance;
}

int main()
{
  auto [grid, start, end] = parse(cin);
  auto [graph, width, height] = make_graph(grid);
  auto distance = solve(graph, width * height, end);

  cout << "Part 1: " << distance[start] << '\n';

  int best = 100000;
  for (int i = 0; i < width * height; i++) {
    if (grid[i / width][i % width] != 0) continue;
    best = min(best, distance[i]);
  }

  cout << "Part 2: " << best << '\n';
}
