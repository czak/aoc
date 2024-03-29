#include "stdafx.h"

istringstream example{R"(....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
)"};

using vec2 = pair<int, int>;

// clang-format off
const vector<vector<vec2>> neighbors{
  { {-1, -1}, { 0, -1}, { 1, -1} }, // north (nw, n, ne)
  { {-1,  1}, { 0,  1}, { 1,  1} }, // south (sw, s, se)
  { {-1, -1}, {-1,  0}, {-1,  1} }, // west  (nw, w, sw)
  { { 1, -1}, { 1,  0}, { 1,  1} }, // east  (ne, e, se)
};

const vector<vec2> headings{
  { 0, -1}, // n
  { 0,  1}, // s
  {-1,  0}, // w
  { 1,  0}, // e
};
// clang-format on

set<vec2> elves{};

set<vec2> stays{};
map<vec2, vector<vec2>> moves{};

bool empty_in(const vec2& p, const vector<vec2>& group)
{
  return (all_of(group.begin(), group.end(), [p](const vec2& d) {
    return elves.count({p.first + d.first, p.second + d.second}) == 0;
  }));
}

void parse(istream& input)
{
  string line;
  int y = 0;
  while (getline(input, line)) {
    int x = 0;
    for (char c : line) {
      if (c == '#') elves.insert({x, y});
      x++;
    }
    y++;
  }
}

void round(int r)
{
  // plan stays & moves
  stays.clear();
  moves.clear();

  for (const vec2& e : elves) {
    if (empty_in(e, neighbors[0]) && empty_in(e, neighbors[1]) && empty_in(e, neighbors[2]) && empty_in(e, neighbors[3])) {
      stays.insert(e);
      continue;
    }

    if (!empty_in(e, neighbors[0]) && !empty_in(e, neighbors[1]) && !empty_in(e, neighbors[2]) && !empty_in(e, neighbors[3])) {
      stays.insert(e);
      continue;
    }

    // check each direction (accounting for round number)
    for (int i = 0; i < 4; i++) {
      int n = (r + i) % 4;
      if (empty_in(e, neighbors[n])) {
        vec2 proposed{
          e.first + headings[n].first,
          e.second + headings[n].second,
        };
        moves[proposed].push_back(e);
        break;
      }
    }
  }

  // execute
  elves = stays;

  for (auto [pos, candidates] : moves) {
    if (candidates.size() == 1) elves.insert(pos);
    else elves.insert(candidates.begin(), candidates.end());
  }
}

tuple<int, int, int, int> area()
{
  auto xrange =
    minmax_element(elves.begin(), elves.end(), [](auto& l, auto& r) { return l.first < r.first; });
  auto yrange = minmax_element(elves.begin(), elves.end(), [](auto& l, auto& r) {
    return l.second < r.second;
  });

  return {xrange.first->first, xrange.second->first, yrange.first->second, yrange.second->second};
}

void dump()
{
  auto [xmin, xmax, ymin, ymax] = area();

  for (int y = ymin; y <= ymax; y++) {
    for (int x = xmin; x <= xmax; x++) {
      if (elves.count({x, y})) cout << '#';
      else cout << '.';
    }
    cout << '\n';
  }
}

int part1()
{
  for (int i = 0; i < 10; i++) {
    round(i);
  }

  auto [xmin, xmax, ymin, ymax] = area();

  return (xmax - xmin + 1) * (ymax - ymin + 1) - elves.size();
}

int part2()
{
  int i = 10;
  set<vec2> prev{};

  while (prev != elves) {
    prev = elves;
    round(i);
    i++;
  }

  return i;
}

int main()
{
  parse(cin);

  cout << "Part 1: " << part1() << '\n';
  cout << "Part 2: " << part2() << '\n';
}
