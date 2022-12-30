#include "stdafx.h"

istringstream example1{R"(#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
)"};

istringstream example2{R"(#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
)"};

const size_t MAX_MINUTES = 300;

using Wind = tuple<int, int, int, int>;   // x, y, dx, dy
using PointInTime = tuple<int, int, int>; // x, y, t

// Make unique by x,y only
template<>
struct std::hash<Wind>
{
  size_t operator()(const Wind& w) const { return 1'000'000 * get<0>(w) + get<1>(w); }
};

template<>
struct std::equal_to<Wind>
{
  bool operator()(const Wind& l, const Wind& r) const
  {
    return get<0>(l) == get<0>(r) && get<1>(l) == get<1>(r);
  }
};

vector<Wind> winds{};
map<PointInTime, vector<PointInTime>> graph{};

int width, height;

void parse(istream& input)
{
  vector<string> grid;
  string line;
  while (getline(input, line)) {
    grid.push_back(line);
  }

  width = grid[0].size() - 2;
  height = grid.size() - 2;

  int x = 0, y = 0;

  for (auto& line : grid) {
    if (line[2] == '#') continue;

    x = 0;

    for (auto& c : line) {
      if (c == '#') continue;

      if (c == '>') winds.push_back({x, y, 1, 0});
      if (c == 'v') winds.push_back({x, y, 0, 1});
      if (c == '<') winds.push_back({x, y, -1, 0});
      if (c == '^') winds.push_back({x, y, 0, -1});

      x++;
    }

    y++;
  }
}

void simulate()
{
  for (auto& [x, y, dx, dy] : winds) {
    x = (x + dx + width) % width;
    y = (y + dy + height) % height;
  }
}

bool is_open(unordered_set<Wind>& winds, int x, int y)
{
  return x >= 0 && x < width && y >= 0 && y < height && winds.count({x, y, 0, 0}) == 0;
}

void make_graph()
{
  unordered_set<Wind> cur{winds.begin(), winds.end()}, last{};
  unsigned int t = 0;

  auto test_move = [&t, &last, &cur](int x, int y, int tx, int ty) {
    if (is_open(last, x, y) && is_open(cur, tx, ty)) {
      graph[{x, y, t - 1}].push_back({tx, ty, t});
    }
  };

  for (t = 1; t <= MAX_MINUTES; t++) {
    last = move(cur);
    simulate();
    cur = {winds.begin(), winds.end()};

    // always can wait at the start
    graph[{0, -1, t - 1}].push_back({0, -1, t});

    // can move into first open spot?
    if (is_open(cur, 0, 0)) graph[{0, -1, t - 1}].push_back({0, 0, t});

    // moves within the grid
    for (int y = 0; y < height; y++) {
      for (int x = 0; x < width; x++) {
        // wait
        test_move(x, y, x, y);
        // right, down, left, up
        test_move(x, y, x + 1, y);
        test_move(x, y, x, y + 1);
        test_move(x, y, x - 1, y);
        test_move(x, y, x, y - 1);
      }
    }

    // can move into finish line?
    if (is_open(last, width - 1, height - 1))
      graph[{width - 1, height - 1, t - 1}].push_back({width - 1, height, t});
  }
}

void make_graph_reverse()
{
  unordered_set<Wind> cur{winds.begin(), winds.end()}, last{};
  unsigned int t = 0;

  auto test_move = [&t, &last, &cur](int x, int y, int tx, int ty) {
    if (is_open(last, x, y) && is_open(cur, tx, ty)) {
      graph[{x, y, t - 1}].push_back({tx, ty, t});
    }
  };

  for (t = 1; t <= MAX_MINUTES; t++) {
    last = move(cur);
    simulate();
    cur = {winds.begin(), winds.end()};

    // always can wait at the start
    graph[{width - 1, height, t - 1}].push_back({width - 1, height, t});

    // can move into first open spot?
    if (is_open(cur, width - 1, height - 1))
      graph[{width - 1, height, t - 1}].push_back({width - 1, height - 1, t});

    // moves within the grid
    for (int y = 0; y < height; y++) {
      for (int x = 0; x < width; x++) {
        // wait
        test_move(x, y, x, y);
        // right, down, left, up
        test_move(x, y, x + 1, y);
        test_move(x, y, x, y + 1);
        test_move(x, y, x - 1, y);
        test_move(x, y, x, y - 1);
      }
    }

    // can move into finish line?
    if (is_open(last, 0, 0)) graph[{0, 0, t - 1}].push_back({0, -1, t});
  }
}

map<PointInTime, int> dist{};

void measure(PointInTime start)
{
  for (unsigned int t = 0; t <= MAX_MINUTES; t++) {
    dist[{0, -1, t}] = 1'000'000;
    dist[{width - 1, height, t}] = 1'000'000;
    for (int y = 0; y < height; y++) {
      for (int x = 0; x < width; x++) {
        dist[{x, y, t}] = 1'000'000;
      }
    }
  }

  set<PointInTime> processed{};
  priority_queue<pair<int, PointInTime>> q;

  dist[start] = 0;
  q.push({0, start});

  while (!q.empty()) {
    PointInTime a = q.top().second;
    q.pop();
    if (processed.count(a)) continue;
    processed.insert(a);

    for (auto b : graph[a]) {
      if (dist[a] + 1 < dist[b]) {
        dist[b] = dist[a] + 1;
        q.push({-dist[b], b});
      }
    }
  }
}

int min_dist(int x, int y)
{
  int best = 1000000;

  for (auto& [p, d] : dist) {
    auto& [px, py, pt] = p;
    if (px == x && py == y && d < 1'000'000) {
      best = min(d, best);
    }
  }

  return best;
}

int main()
{
  parse(cin);

  // 221
  // 258
  for (int i = 0; i < 221 + 258; i++)
    simulate();

  make_graph();
  measure({0, -1, 0});

  cout << "Part 2: " << 221 + 258 + min_dist(width - 1, height) << '\n';
}
