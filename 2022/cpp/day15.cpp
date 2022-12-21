#include "stdafx.h"
#include "utils.h"

istringstream example{R"(Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
)"};

const regex REGEX{"Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)"};

using sensor = tuple<int, int, int>;
using area = pair<int, int>;

vector<sensor> parse(istream& in)
{
  string data{istreambuf_iterator{in}, {}};
  return parse_to<vector<sensor>>(data, REGEX, [](const auto& sm) {
    int x = stoi(sm[1]);
    int y = stoi(sm[2]);
    int bx = stoi(sm[3]);
    int by = stoi(sm[4]);
    return sensor{x, y, abs(x - bx) + abs(y - by)};
  });
}

area coverage(const vector<sensor>& sensors, int row)
{
  vector<area> overlaps{};

  for (auto& [x, y, range] : sensors) {
    int dy = abs(y - row);
    if (dy > range) continue;
    overlaps.push_back({x - (range - dy), x + (range - dy)});
  }

  sort(overlaps.begin(), overlaps.end());

  auto [l, r] = overlaps.front();
  for (auto& [a, b] : overlaps) {
    if (a == r + 2) throw a - 1; // gap
    l = min(l, a);
    r = max(r, b);
  }

  return {l, r};
}

int part1(const vector<sensor>& sensors, int row)
{
  auto area = coverage(sensors, row);
  return area.second - area.first;
}

long long part2(const vector<sensor>& sensors, int limit)
{
  for (int y = 0; y <= limit; y++) {
    try {
      coverage(sensors, y);
    } catch (int x) {
      return 4000000LL * x + y;
    }
  }
  throw new runtime_error{"no beacon"};
}

int main()
{
  auto sensors = parse(cin);

  cout << "Part 1: " << part1(sensors, 2000000) << '\n';
  cout << "Part 2: " << part2(sensors, 4000000) << '\n';
}
