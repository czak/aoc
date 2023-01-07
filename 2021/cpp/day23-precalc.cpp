#include "stdafx.h"

// 01·2·3·4·56
//   7 b f 3
//   8 c 0 4
//   9 d 1 5
//   a e 2 6

// adj[from] = vector<pair<to, weight>>
unordered_map<long, vector<pair<long, long>>> adj{
  // corridor
  {0x00, {{0x01, 1}}},
  {0x01, {{0x00, 1}, {0x02, 2}, {0x07, 2}}},
  {0x02, {{0x01, 2}, {0x03, 2}, {0x07, 2}, {0x0B, 2}}},
  {0x03, {{0x02, 2}, {0x04, 2}, {0x0B, 2}, {0x0F, 2}}},
  {0x04, {{0x03, 2}, {0x05, 2}, {0x0F, 2}, {0x13, 2}}},
  {0x05, {{0x04, 2}, {0x06, 1}, {0x13, 2}}},
  {0x06, {{0x05, 1}}},
  // room A
  {0x07, {{0x01, 2}, {0x02, 2}, {0x08, 1}}},
  {0x08, {{0x07, 1}, {0x09, 1}}},
  {0x09, {{0x08, 1}, {0x0A, 1}}},
  {0x0A, {{0x09, 1}}},
  // room B
  {0x0B, {{0x02, 2}, {0x03, 2}, {0x0C, 1}}},
  {0x0C, {{0x0B, 1}, {0x0D, 1}}},
  {0x0D, {{0x0C, 1}, {0x0E, 1}}},
  {0x0E, {{0x0D, 1}}},
  // room C
  {0x0F, {{0x03, 2}, {0x04, 2}, {0x10, 1}}},
  {0x10, {{0x0F, 1}, {0x11, 1}}},
  {0x11, {{0x10, 1}, {0x12, 1}}},
  {0x12, {{0x11, 1}}},
  // room D
  {0x13, {{0x04, 2}, {0x05, 2}, {0x14, 1}}},
  {0x14, {{0x13, 1}, {0x15, 1}}},
  {0x15, {{0x14, 1}, {0x16, 1}}},
  {0x16, {{0x15, 1}}},
};


// path does NOT INCLUDE 'from' but INCLUDES 'to'
using path = pair<vector<long>, long>;

// paths[from][to] = optional<path>
array<array<optional<path>, 0x16 + 1>, 0x16 + 1> paths{};

path find_path(long from, long to)
{
  static bool visited[0x16 + 1]{};

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
  for (long from = 0x0; from <= 0x16; from++) {
    for (long to = 0x0; to <= 0x16; to++) {
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

int main()
{
  precalc();

  for (long from = 0x0; from <= 0x16; from++) {
    printf("{ ");
    for (long to = 0x0; to <= 0x16; to++) {
      const optional<path>& val = paths[from][to];
      if (val) {
        cout << "path";
        dbg::pretty_print(cout, *paths[from][to]);
      }
      else {
        printf("nullopt");
      }
      if (to != 0x16) printf(", ");
    }
    printf(" },\n");
  }
}
