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

using Cube = tuple<int, int, int>;
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

template<>
struct std::hash<Cube>
{
  size_t operator()(const Cube& c) const
  {
    return 1'000'000 * get<0>(c) + 1'000 * get<1>(c) + 3 * get<2>(c);
  }
};

unordered_map<Wall, int> walls{};
unordered_set<Cube> cubes{};
unordered_set<Cube> steam{};

int xmin = 1000, xmax = -1000;
int ymin = 1000, ymax = -1000;
int zmin = 1000, zmax = -1000;

void insert(Wall w)
{
  walls.insert({w, 0});
  walls[w]++;
}

void add_walls(Cube c)
{
  auto [x, y, z] = c;
  insert({Orientation::XY, x, y, z - 1});
  insert({Orientation::XY, x, y, z});
  insert({Orientation::XZ, x, y - 1, z});
  insert({Orientation::XZ, x, y, z});
  insert({Orientation::YZ, x - 1, y, z});
  insert({Orientation::YZ, x, y, z});
}

int part1()
{
  for (auto cube : cubes)
    add_walls(cube);

  return count_if(walls.begin(), walls.end(), [](auto& w) {
    return w.second == 1;
  });
}

void fill(int x, int y, int z)
{
  if (x < xmin - 1 || x > xmax + 1 || y < ymin - 1 || y > ymax + 1 || z < zmin - 1 || z > zmax + 1)
    return;
  if (steam.count({x, y, z})) return;
  if (cubes.count({x, y, z})) return;

  steam.insert({x, y, z});

  fill(x, y, z + 1);
  fill(x, y, z - 1);
  fill(x, y + 1, z);
  fill(x, y - 1, z);
  fill(x + 1, y, z);
  fill(x - 1, y, z);
}

int part2()
{
  fill(xmin - 1, ymin - 1, zmin - 1);

  walls.clear();
  for (int x = xmin; x <= xmax; x++) {
    for (int y = ymin; y <= ymax; y++) {
      for (int z = zmin; z <= zmax; z++) {
        if (!steam.count({x, y, z})) {
          add_walls({x, y, z});
        }
      }
    }
  }

  return count_if(walls.begin(), walls.end(), [](auto& w) {
    return w.second == 1;
  });
}

int main()
{
  string s{istreambuf_iterator{cin}, {}};

  cubes = parse_to<unordered_set<Cube>>(
    s, regex{"(\\d+),(\\d+),(\\d+)"},
    [](auto sm) {
      int x = stoi(sm[1]);
      int y = stoi(sm[2]);
      int z = stoi(sm[3]);
      xmin = min(xmin, x);
      xmax = max(xmax, x);
      ymin = min(ymin, y);
      ymax = max(ymax, y);
      zmin = min(zmin, z);
      zmax = max(zmax, z);
      return tuple<int, int, int>{x, y, z};
    }
  );

  cout << "Part 1: " << part1() << '\n';
  cout << "Part 2: " << part2() << '\n';
}
