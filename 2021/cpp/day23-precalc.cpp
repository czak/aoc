#include "stdafx.h"

// adj[from] = vector<pair<to, weight>>
unordered_map<long, vector<pair<long, long>>> adj{
  // corridor
  {0x0, {{0x1, 1}}},
  {0x1, {{0x0, 1}, {0x2, 2}, {0x7, 2}}},
  {0x2, {{0x1, 2}, {0x3, 2}, {0x7, 2}, {0x9, 2}}},
  {0x3, {{0x2, 2}, {0x4, 2}, {0x9, 2}, {0xB, 2}}},
  {0x4, {{0x3, 2}, {0x5, 2}, {0xB, 2}, {0xD, 2}}},
  {0x5, {{0x4, 2}, {0x6, 1}, {0xD, 2}}},
  {0x6, {{0x5, 1}}},
  // room A
  {0x7, {{0x1, 2}, {0x2, 2}, {0x8, 1}}},
  {0x8, {{0x7, 1}}},
  // room B
  {0x9, {{0x2, 2}, {0x3, 2}, {0xA, 1}}},
  {0xA, {{0x9, 1}}},
  // room C
  {0xB, {{0x3, 2}, {0x4, 2}, {0xC, 1}}},
  {0xC, {{0xB, 1}}},
  // room D
  {0xD, {{0x4, 2}, {0x5, 2}, {0xE, 1}}},
  {0xE, {{0xD, 1}}},
};


// path does NOT INCLUDE 'from' but INCLUDES 'to'
using path = pair<vector<long>, long>;

// paths[from][to] = optional<path>
array<array<optional<path>, 15>, 15> paths{};

// 01·2·3·4·56
//   7 9 b d
//   8 a c e

path find_path(long from, long to)
{
  static bool visited[15]{};

  if (from == to) return {{}, 0};

  path best = {{}, 1'000'000};

  for (auto& [next, w] : adj[from]) {
    if (visited[next]) continue;
    visited[next] = true;

    // is this better than current best?
    path p = find_path(next, to);
    if (p.second + w < best.second) {
      best = move(p);
      best.first.insert(best.first.begin(), 1, next);
      best.second += w;
    }

    visited[next] = false;
  }

  return best;
}

void precalc()
{
  for (long from = 0x0; from <= 0xE; from++) {
    for (long to = 0x0; to <= 0xE; to++) {
      // invalid paths
      // - staying in the same location
      // - from corridor to corridor
      // - from room to room
      if (from == to || (from <= 0x6 && to <= 0x6) || (from > 0x6 && to > 0x6)) {
        paths[from][to] = nullopt;
        continue;
      }

      paths[from][to] = find_path(from, to);
    }
  }
}

// 01·2·3·4·56
//   7 9 b d
//   8 a c e

int main()
{
  precalc();

  for (long from = 0x0; from <= 0xE; from++) {
    printf("{ ");
    for (long to = 0x0; to <= 0xE; to++) {
      const optional<path>& val = paths[from][to];
      if (val) {
        cout << "path";
        dbg::pretty_print(cout, *paths[from][to]);
      }
      else {
        printf("nullopt");
      }
      if (to != 0xE) printf(", ");
    }
    printf(" },\n");
  }
}
