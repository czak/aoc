#include "stdafx.h"
#include "utils.h"

istringstream example{R"(498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
)"};

using vec2 = pair<int, int>;
using cave = unordered_set<vec2>;

template<>
struct std::hash<vec2>
{
  size_t operator()(const vec2& v) const { return v.second * 1000 + v.first; }
};

cave parse(istream& input)
{
  cave c{};
  regex r{"(\\d+),(\\d+)"};

  string line;
  while (getline(input, line)) {
    auto coords = parse_to<vector<vec2>>(line, r, [](auto& sm) { return vec2{stoi(sm[1]), stoi(sm[2])}; });
    for (size_t i = 1; i < coords.size(); i++) {
      auto [sx, sy] = coords[i - 1];
      auto [ex, ey] = coords[i];
      int dx = sx == ex ? 0 : (ex - sx) / abs(ex - sx);
      int dy = sy == ey ? 0 : (ey - sy) / abs(ey - sy);
      for (int x = sx, y = sy; x != ex || y != ey; x += dx, y += dy) {
        c.insert({x, y});
      }
      c.insert({ex, ey});
    }
  }

  return c;
}

tuple<int, int, int, int> dimensions(const cave& c)
{
  auto xrange = minmax_element(c.begin(), c.end(), [](auto& l, auto& r) { return l.first < r.first; });
  auto yrange = minmax_element(c.begin(), c.end(), [](auto& l, auto& r) { return l.second < r.second; });
  return {
    xrange.first->first,
    xrange.second->first,
    yrange.first->second,
    yrange.second->second,
  };
}

void draw(const cave& c)
{
  auto [xmin, xmax, ymin, ymax] = dimensions(c);
  for (int y = min(ymin, 0); y <= max(ymax, 0); y++) {
    for (int x = min(xmin, 500); x <= max(xmax, 500); x++) {
      if (x == 500 && y == 0) printf("+");
      else if (c.count({x, y})) printf("#");
      else printf(".");
    }
    printf("\n");
  }
}

optional<vec2> open(const cave& c, int x, int y)
{
  if (!c.count({x, y + 1})) return vec2{x, y + 1};
  if (!c.count({x - 1, y + 1})) return vec2{x - 1, y + 1};
  if (!c.count({x + 1, y + 1})) return vec2{x + 1, y + 1};
  return {};
}

int simulate(cave c, function<bool(int, int, int)> cond)
{
  int count = 0;
  int ymax = get<3>(dimensions(c)) + 1;
  while (true) {
    int x = 500, y = 0;
    while (y < ymax && open(c, x, y)) {
      tie(x, y) = open(c, x, y).value();
    }
    c.insert({x, y});
    count++;
    if (cond(x, y, ymax)) return count;
  }
}

int main()
{
  cave c = parse(cin);

  cout << "Part 1: " << simulate(c, [](int x, int y, int ymax) { return y == ymax; }) - 1 << '\n';
  cout << "Part 2: " << simulate(c, [](int x, int y, int ymax) { return x == 500 && y == 0; }) << '\n';
}
