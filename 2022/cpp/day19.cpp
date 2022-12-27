#include "stdafx.h"

struct Resource
{
  int ore;
  int clay;
  int obsidian;
  int geode;
};

ostream& operator<<(ostream& out, const Resource& r)
{
  out << "ORE:      " << r.ore << '\n';
  out << "CLAY:     " << r.clay << '\n';
  out << "OBSIDIAN: " << r.obsidian << '\n';
  out << "GEODE:    " << r.geode << '\n';
  return out;
}

enum class RobotType
{
  Ore,
  Clay,
  Obsidian,
  Geode,
};

using Blueprint = map<RobotType, Resource>;

// Blueprint 1
Blueprint example1{
  {RobotType::Ore,      {4, 0, 0} },
  {RobotType::Clay,     {2, 0, 0} },
  {RobotType::Obsidian, {3, 14, 0}},
  {RobotType::Geode,    {2, 0, 7} },
};

// Blueprint 2
Blueprint example2{
  {RobotType::Ore,      {2, 0, 0} },
  {RobotType::Clay,     {3, 0, 0} },
  {RobotType::Obsidian, {3, 8, 0} },
  {RobotType::Geode,    {3, 0, 12}},
};

Blueprint b;

vector<Blueprint> blueprints{
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 7, 0}},
   {RobotType::Geode, {4, 0, 11}}},
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {2, 20, 0}},
   {RobotType::Geode, {2, 0, 20}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 14, 0}},
   {RobotType::Geode, {4, 0, 8}} },
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 10, 0}},
   {RobotType::Geode, {2, 0, 13}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 5, 0}},
   {RobotType::Geode, {3, 0, 7}} },
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 11, 0}},
   {RobotType::Geode, {4, 0, 12}}},
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 16, 0}},
   {RobotType::Geode, {3, 0, 15}}},
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {4, 19, 0}},
   {RobotType::Geode, {4, 0, 7}} },
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {2, 13, 0}},
   {RobotType::Geode, {2, 0, 9}} },
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 14, 0}},
   {RobotType::Geode, {4, 0, 15}}},
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {2, 14, 0}},
   {RobotType::Geode, {3, 0, 17}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {4, 20, 0}},
   {RobotType::Geode, {2, 0, 15}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {2, 17, 0}},
   {RobotType::Geode, {3, 0, 11}}},
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 12, 0}},
   {RobotType::Geode, {3, 0, 17}}},
  {{RobotType::Ore, {2, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {3, 18, 0}},
   {RobotType::Geode, {2, 0, 19}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {4, 11, 0}},
   {RobotType::Geode, {3, 0, 15}}},
  {{RobotType::Ore, {2, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 19, 0}},
   {RobotType::Geode, {4, 0, 8}} },
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {4, 8, 0}},
   {RobotType::Geode, {3, 0, 7}} },
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 10, 0}},
   {RobotType::Geode, {2, 0, 7}} },
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 17, 0}},
   {RobotType::Geode, {4, 0, 16}}},
  {{RobotType::Ore, {2, 0, 0}},
   {RobotType::Clay, {2, 0, 0}},
   {RobotType::Obsidian, {2, 20, 0}},
   {RobotType::Geode, {2, 0, 14}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 20, 0}},
   {RobotType::Geode, {2, 0, 8}} },
  {{RobotType::Ore, {2, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 19, 0}},
   {RobotType::Geode, {4, 0, 13}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {2, 20, 0}},
   {RobotType::Geode, {3, 0, 9}} },
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {3, 7, 0}},
   {RobotType::Geode, {3, 0, 20}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {3, 7, 0}},
   {RobotType::Geode, {2, 0, 7}} },
  {{RobotType::Ore, {3, 0, 0}},
   {RobotType::Clay, {3, 0, 0}},
   {RobotType::Obsidian, {2, 13, 0}},
   {RobotType::Geode, {3, 0, 12}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {4, 9, 0}},
   {RobotType::Geode, {2, 0, 20}}},
  {{RobotType::Ore, {2, 0, 0}},
   {RobotType::Clay, {2, 0, 0}},
   {RobotType::Obsidian, {2, 10, 0}},
   {RobotType::Geode, {2, 0, 11}}},
  {{RobotType::Ore, {4, 0, 0}},
   {RobotType::Clay, {4, 0, 0}},
   {RobotType::Obsidian, {2, 14, 0}},
   {RobotType::Geode, {4, 0, 19}}},
};

using Key = tuple<int, int, int, int, int, int, int, int, int>;

template<>
struct std::hash<Key>
{
  size_t operator()(const Key& k) const
  {
    auto [t, ro, rc, rob, rg, mo, mc, mob, mg] = k;
    return 100 * t + 10000 * ro + 1000000 * rc + 100000000 * rob + 10000000000 * rg +
           1000000000000 * mo + 100000000000000 * mc + 10000000000000000 * mob +
           1000000000000000000 * mg;
  }
};

unordered_map<Key, int> memo{};

int search(int time, Resource robots, Resource materials)
{
  auto time_to_build = [time, &m = materials, &r = robots](RobotType rt) -> optional<int> {
    auto cost = b[rt];

    // can build in the future
    auto ore_time = fmax(ceil(float(cost.ore - m.ore) / r.ore), 0) + 1;
    auto clay_time = fmax(ceil(float(cost.clay - m.clay) / r.clay), 0) + 1;
    auto obsidian_time = fmax(ceil(float(cost.obsidian - m.obsidian) / r.obsidian), 0) + 1;

    // if any of these is inf, then impossible to build this one next
    if (isinf(ore_time) || isinf(clay_time) || isinf(obsidian_time)) return {};

    // if any of these if more than time left, also impossible to build
    if (ore_time > time || clay_time > time || obsidian_time > time) return {};

    return max(max(ore_time, clay_time), obsidian_time);
  };

  if (time <= 1) return materials.geode + time * robots.geode;

  vector<RobotType> candidates{};

  if (time_to_build(RobotType::Ore)) candidates.push_back(RobotType::Ore);
  if (time_to_build(RobotType::Clay)) candidates.push_back(RobotType::Clay);
  if (time_to_build(RobotType::Obsidian)) candidates.push_back(RobotType::Obsidian);
  if (time_to_build(RobotType::Geode)) candidates.push_back(RobotType::Geode);

  // no more robots to build
  if (candidates.empty()) {
    return materials.geode + time * robots.geode;
  }

  Key key = {time,          robots.ore,     robots.clay,        robots.obsidian, robots.geode,
             materials.ore, materials.clay, materials.obsidian, materials.geode};
  if (memo.count(key)) return memo[key];

  int best = -1;

  for (auto rt : candidates) {
    int time_spent = *time_to_build(rt); // assume present and > 0 because checked above
    int time_left = time - time_spent;

    auto cost = b[rt];

    // clang-format off
    best = max(best, search(
      time_left,
      {
        robots.ore + (rt == RobotType::Ore ? 1 : 0),
        robots.clay + (rt == RobotType::Clay ? 1 : 0),
        robots.obsidian + (rt == RobotType::Obsidian ? 1 : 0),
        robots.geode + (rt == RobotType::Geode ? 1 : 0)
      },
      {
        materials.ore + time_spent * robots.ore - cost.ore,
        materials.clay + time_spent * robots.clay - cost.clay,
        materials.obsidian + time_spent * robots.obsidian - cost.obsidian,
        materials.geode + time_spent * robots.geode,
      }
    ));
    // clang-format on
  }

  memo[key] = best;
  return best;
}

int part1()
{
  int sum = 0;

  for (size_t i = 0; i < blueprints.size(); i++) {
    b = blueprints[i];
    memo.clear();
    int geodes = search(24, {1, 0, 0, 0}, {0, 0, 0, 0});
    sum += (i + 1) * geodes;
  }

  return sum;
}

int part2()
{
  memo.clear();
  b = blueprints[0];
  int g1 = search(32, {1, 0, 0, 0}, {0, 0, 0, 0});

  memo.clear();
  b = blueprints[1];
  int g2 = search(32, {1, 0, 0, 0}, {0, 0, 0, 0});

  memo.clear();
  b = blueprints[2];
  int g3 = search(32, {1, 0, 0, 0}, {0, 0, 0, 0});

  return g1 * g2 * g3;
}

int main()
{
  cout << "Part 1: " << flush << part1() << '\n';
  cout << "Part 2 (wait for it...): " << flush << part2() << '\n';
}
