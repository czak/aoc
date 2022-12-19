#include "stdafx.h"

istringstream example{R"(Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi)"};

using Grid = vector<vector<int>>;
using Point = pair<int, int>;
using Graph = unordered_map<int, unordered_set<int>>; // 100*y + x

tuple<Grid, Point, Point> parse(istream& input)
{
  int x = 0, y = 0;
  Grid grid{{}};
  Point start{}, end{};

  char c;
  while (example.get(c)) {
    if (c == '\n') {
      grid.push_back({});
      x = 0;
      y++;
    } else {
      auto& row = grid.back();
      if (c == 'S') {
        start = {x, y};
        row.push_back(0);
      } else if (c == 'E') {
        end = {x, y};
        row.push_back(25);
      } else {
        row.push_back(c - 'a');
      }
      x++;
    }
  }

  return {grid, start, end};
}

Graph make_graph(const Grid& grid)
{
  Graph g{};
  for (int y = 0; y < grid.size(); y++) {
    for (int x = 0; x < grid[0].size(); x++) {
      unordered_set<int> vs{};
      int loc = 100 * y + x;
      int elev = grid[y][x];

      // up
      if (y > 0 && grid[y - 1][x] - elev <= 1) vs.insert(loc - 100);

      // down
      if (y < grid.size() - 1 && grid[y + 1][x] - elev <= 1) vs.insert(loc + 100);

      // left
      if (x > 0 && grid[y][x - 1] - elev <= 1) vs.insert(loc - 1);

      // right
      if (x < grid[0].size() - 1 && grid[y][x + 1] - elev <= 1) vs.insert(loc + 1);

      g[loc] = vs;
    }
  }
  return g;
}

int main()
{
  auto [grid, start, end] = parse(example);
  auto graph = make_graph(grid);

  dbg(grid);
  dbg(start);
  dbg(end);

  dbg(graph[304]);
}
