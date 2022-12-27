#include "stdafx.h"

istringstream example{
  R"(Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
)"};

struct Cost
{
  int ore;
  int clay;
  int obsidian;
};

struct Blueprint
{
  Cost ore_robot;
  Cost clay_robot;
  Cost obsidian_robot;
  Cost geode_robot;
};

struct Resource
{
  int ore;
  int clay;
  int obsidian;
  int geode;
};

enum class Robot
{
  Ore,
  Clay,
  Obsidian,
  Geode,
};

// Blueprint 1
Blueprint b{
  {4, 0,  0},
  {2, 0,  0},
  {3, 14, 0},
  {2, 0,  7},
};

// step = time, robot being built
using Path = vector<tuple<int, Robot>>;

void search(int time, Resource materials, Resource robots)
{
  static Path path{};

  // can build ore robot?
  // - has enough resources already OR
  // - has the robots which provide them
  auto time_to_build = [&m = materials, &r = robots](Cost c) {
    if (m.ore >= c.ore && m.clay >= c.clay && m.obsidian >= c.obsidian)
      return 1;

    return -1;
  };

  vector<Robot> candidates{};
}

int main()
{
  search(24, {1, 0, 0, 0}, {0, 0, 0, 0});
}
