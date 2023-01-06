#include "stdafx.h"

using state = array<long, 8>;

unordered_map<long, long> MEM;
const state solved{0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE};
const state multipliers{1, 1, 10, 10, 100, 100, 1000, 1000};

// clang-format off
// bi-directional edges
vector<pair<size_t, size_t>> edges{
  {0x0, 0x1}, {0x1, 0x2}, {0x2, 0x3}, {0x3, 0x4}, {0x4, 0x5}, {0x5, 0x6},
  {0x1, 0x7}, {0x2, 0x7}, {0x7, 0x8}, // room A
  {0x2, 0x9}, {0x3, 0x9}, {0x9, 0xA}, // room B
  {0x3, 0xB}, {0x4, 0xB}, {0xB, 0xC}, // room C
  {0x4, 0xD}, {0x5, 0xD}, {0xD, 0xE}, // room D
};
// clang-format on

// valid positions
// 01·2·3·4·56
//   7 9 b d
//   8 a c e

// cost of moving i-th amphipod in s to 'to' location
// - includes multiplier
long distance(const state& s, long i, long to)
{
  long from = s[i];

  // Invalid moves
  // - trying to move to the same location
  if (from == to) return -1;

  // - moving from one spot in corridor to another in corridor
  if (from <= 0x6 && to <= 0x6) return -1;

  // - moving from a room to another room (or within a room)
  if (from > 0x6 && to > 0x6) return -1;

  // - moving into a wrong room (e.g. index i==1 (A) tries to move to 0x9)
  // A
  if ((i == 0 || i == 1) && (to == 0x9 || to == 0xA || to == 0xB || to == 0xC || to == 0xD || to == 0xE))
    return -1;
  // B
  if ((i == 2 || i == 3) && (to == 0x7 || to == 0x8 || to == 0xB || to == 0xC || to == 0xD || to == 0xE))
    return -1;
  // C
  if ((i == 4 || i == 5) && (to == 0x7 || to == 0x8 || to == 0x9 || to == 0xA || to == 0xD || to == 0xE))
    return -1;
  // D
  if ((i == 6 || i == 7) && (to == 0x7 || to == 0x8 || to == 0x9 || to == 0xA || to == 0xB || to == 0xC))
    return -1;

  // cave occupancy
  // cave[0x7] == -1 means that 1st chamber of cave A is vacant
  // cave[0x8] ==  1 means that 2nd chamber of cave A is inhabited by second amphipod A
  long cave[15]{-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1};
  for (long j = 0; j < 8; j++) {
    cave[s[j]] = j;
  }

  // - moving into 1st chamber of a correct room but it's currently
  //   - empty OR
  //   - occupied by a wrong amphipod
  // A (0 or 1) trying to move to 0x7 when 0x8 is empty or occupied by non-A
  if (i == 0 && to == 0x7 && cave[0x8] != 1) return -1;
  if (i == 1 && to == 0x7 && cave[0x8] != 0) return -1;
  // B (2 or 3) trying to move to 0x9 when 0xA is empty or occupied by non-B
  if (i == 2 && to == 0x9 && cave[0xA] != 3) return -1;
  if (i == 3 && to == 0x9 && cave[0xA] != 2) return -1;
  // C (4 or 5) trying to move to 0xB when 0xC is empty or occupied by non-C
  if (i == 4 && to == 0xB && cave[0xC] != 5) return -1;
  if (i == 5 && to == 0xB && cave[0xC] != 4) return -1;
  // D (6 or 7) trying to move to 0xD when 0xE is empty or occupied by non-D
  if (i == 6 && to == 0xD && cave[0xE] != 7) return -1;
  if (i == 7 && to == 0xD && cave[0xE] != 6) return -1;

  // --- from here on we're dealing with valid moves, but the path can be blocked ---

  // precalculate paths for all valid moves

  // take the appropriate path paths[from][to] (don't copy)
  // path SHOULD NOT include the starting point but SHOULD include the ending point
  // for every location in the path check that cave[location] == 0
  // - if any is != 0, then path is blocked => return -1
  // - if all are == 0, then path is valid => return distance(path) * multiplier

  return 0;
}

// cost of transitioning to a fully-sorted state (all in their respective rooms)
long solve(state& s)
{
  // FIXME: s can be in different order
  //        add is_solved(const state&) instead
  if (s == solved) return 0;

  long best = 1'000'000;

  for (long i = 0; i < 8; i++) {
    for (long target = 0; target <= 0xE; target++) {
      long cost = distance(s, i, target);
      if (cost < 0) continue; // negative means move is not valid

      //
      long tmp = s[i];
      s[i] = target;
      best = min(best, solve(s) + cost);
      s[i] = tmp;
    }
  }

  return best;
}

int main()
{
  // solve(9, 8, 0, 0, 0, 0, 0, 0);
  state s{9, 8, 0, 0, 0, 0, 0, 0};
  solve(s);
}
