#include "stdafx.h"
#include "utils.h"

istringstream example{R"(2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
)"};

enum class Orientation
{
  XY = 0,
  XZ = 1,
  YZ = 2,
};

using Wall = tuple<Orientation, int, int, int>;

template<>
struct std::hash<Wall>
{
  size_t operator()(const Wall& w) const
  {
    return 1'000'000 * get<1>(w) + 1'000 * get<2>(w) + 3 * get<3>(w) +
           static_cast<int>(get<0>(w));
  }
};

unordered_map<Wall, int> walls{};

void insert(Wall w)
{
  walls.insert({w, 0});
  walls[w]++;
}

int part1()
{
  return count_if(walls.begin(), walls.end(),
                  [](auto& w) { return w.second == 1; });
}

int main()
{
  string s{istreambuf_iterator{cin}, {}};
  auto cubes = parse_to<vector<tuple<int, int, int>>>(
    s, regex{"(\\d+),(\\d+),(\\d+)"}, [](auto sm) {
      return tuple<int, int, int>{stoi(sm[1]), stoi(sm[2]), stoi(sm[3])};
    });

  for (auto [x, y, z] : cubes) {
    insert({Orientation::XY, x, y, z - 1});
    insert({Orientation::XY, x, y, z});
    insert({Orientation::XZ, x, y - 1, z});
    insert({Orientation::XZ, x, y, z});
    insert({Orientation::YZ, x - 1, y, z});
    insert({Orientation::YZ, x, y, z});
  }

  cout << "Part 1: " << part1() << '\n';
}
