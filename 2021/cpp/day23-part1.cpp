#include "stdafx.h"

using state = array<long, 8>;
using path = pair<vector<long>, long>; // path does NOT INCLUDE 'from' but INCLUDES 'to'

// 01·2·3·4·56
//   7 9 b d
//   8 a c e

// see day23-precalc.cpp
const array<array<optional<path>, 15>, 15> paths{
  {
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{1, 7}, 3}, path{{1, 7, 8}, 4}, path{{1, 2, 9}, 5}, path{{1, 2, 9, 10}, 6}, path{{1, 2, 3, 11}, 7}, path{{1, 2, 3, 11, 12}, 8}, path{{1, 2, 3, 4, 13}, 9}, path{{1, 2, 3, 4, 13, 14}, 10} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{7}, 2}, path{{7, 8}, 3}, path{{2, 9}, 4}, path{{2, 9, 10}, 5}, path{{2, 3, 11}, 6}, path{{2, 3, 11, 12}, 7}, path{{2, 3, 4, 13}, 8}, path{{2, 3, 4, 13, 14}, 9} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{7}, 2}, path{{7, 8}, 3}, path{{9}, 2}, path{{9, 10}, 3}, path{{3, 11}, 4}, path{{3, 11, 12}, 5}, path{{3, 4, 13}, 6}, path{{3, 4, 13, 14}, 7} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{2, 7}, 4}, path{{2, 7, 8}, 5}, path{{9}, 2}, path{{9, 10}, 3}, path{{11}, 2}, path{{11, 12}, 3}, path{{4, 13}, 4}, path{{4, 13, 14}, 5} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{3, 2, 7}, 6}, path{{3, 2, 7, 8}, 7}, path{{3, 9}, 4}, path{{3, 9, 10}, 5}, path{{11}, 2}, path{{11, 12}, 3}, path{{13}, 2}, path{{13, 14}, 3} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{4, 3, 2, 7}, 8}, path{{4, 3, 2, 7, 8}, 9}, path{{4, 3, 9}, 6}, path{{4, 3, 9, 10}, 7}, path{{4, 11}, 4}, path{{4, 11, 12}, 5}, path{{13}, 2}, path{{13, 14}, 3} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{5, 4, 3, 2, 7}, 9}, path{{5, 4, 3, 2, 7, 8}, 10}, path{{5, 4, 3, 9}, 7}, path{{5, 4, 3, 9, 10}, 8}, path{{5, 4, 11}, 5}, path{{5, 4, 11, 12}, 6}, path{{5, 13}, 3}, path{{5, 13, 14}, 4} },
    { path{{1, 0}, 3}, path{{1}, 2}, path{{2}, 2}, path{{2, 3}, 4}, path{{2, 3, 4}, 6}, path{{2, 3, 4, 5}, 8}, path{{2, 3, 4, 5, 6}, 9}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{7, 1, 0}, 4}, path{{7, 1}, 3}, path{{7, 2}, 3}, path{{7, 2, 3}, 5}, path{{7, 2, 3, 4}, 7}, path{{7, 2, 3, 4, 5}, 9}, path{{7, 2, 3, 4, 5, 6}, 10}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{2, 1, 0}, 5}, path{{2, 1}, 4}, path{{2}, 2}, path{{3}, 2}, path{{3, 4}, 4}, path{{3, 4, 5}, 6}, path{{3, 4, 5, 6}, 7}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{9, 2, 1, 0}, 6}, path{{9, 2, 1}, 5}, path{{9, 2}, 3}, path{{9, 3}, 3}, path{{9, 3, 4}, 5}, path{{9, 3, 4, 5}, 7}, path{{9, 3, 4, 5, 6}, 8}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{3, 2, 1, 0}, 7}, path{{3, 2, 1}, 6}, path{{3, 2}, 4}, path{{3}, 2}, path{{4}, 2}, path{{4, 5}, 4}, path{{4, 5, 6}, 5}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{11, 3, 2, 1, 0}, 8}, path{{11, 3, 2, 1}, 7}, path{{11, 3, 2}, 5}, path{{11, 3}, 3}, path{{11, 4}, 3}, path{{11, 4, 5}, 5}, path{{11, 4, 5, 6}, 6}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{4, 3, 2, 1, 0}, 9}, path{{4, 3, 2, 1}, 8}, path{{4, 3, 2}, 6}, path{{4, 3}, 4}, path{{4}, 2}, path{{5}, 2}, path{{5, 6}, 3}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{13, 4, 3, 2, 1, 0}, 10}, path{{13, 4, 3, 2, 1}, 9}, path{{13, 4, 3, 2}, 7}, path{{13, 4, 3}, 5}, path{{13, 4}, 3}, path{{13, 5}, 3}, path{{13, 5, 6}, 4}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
  },
};

const state multipliers{1, 1, 10, 10, 100, 100, 1000, 1000};

// cost of moving i-th amphipod in s to 'to' location
// - includes multiplier
long cost(const state& s, long i, long to)
{
  const long from = s[i];
  const auto& p = paths[from][to];

  // Invalid moves
  // - no path
  if (!p) return -1;

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

  // - moving into 1st chamber of a correct room but 2nd chamber is currently
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

  // - trying to move out of a "locked correct" state in correct room
  if (i == 0 && s[i] == 0x8) return -1;
  if (i == 0 && s[i] == 0x7 && cave[0x8] == 1) return -1;
  if (i == 1 && s[i] == 0x8) return -1;
  if (i == 1 && s[i] == 0x7 && cave[0x8] == 0) return -1;

  if (i == 2 && s[i] == 0xA) return -1;
  if (i == 2 && s[i] == 0x9 && cave[0xA] == 3) return -1;
  if (i == 3 && s[i] == 0xA) return -1;
  if (i == 3 && s[i] == 0x9 && cave[0xA] == 2) return -1;

  if (i == 4 && s[i] == 0xC) return -1;
  if (i == 4 && s[i] == 0xB && cave[0xC] == 5) return -1;
  if (i == 5 && s[i] == 0xC) return -1;
  if (i == 5 && s[i] == 0xB && cave[0xC] == 4) return -1;

  if (i == 6 && s[i] == 0xE) return -1;
  if (i == 6 && s[i] == 0xD && cave[0xE] == 7) return -1;
  if (i == 7 && s[i] == 0xE) return -1;
  if (i == 7 && s[i] == 0xD && cave[0xE] == 6) return -1;

  // --- from here on we're dealing with valid moves, but the path can be blocked ---

  for (auto& step : p->first) {
    if (cave[step] != -1) return -1;
  }

  return p->second * multipliers[i];
}

long key(const state& s)
{
  return
    (min(s[0], s[1]) << 28) +
    (max(s[0], s[1]) << 24) +
    (min(s[2], s[3]) << 20) +
    (max(s[2], s[3]) << 16) +
    (min(s[4], s[5]) << 12) +
    (max(s[4], s[5]) <<  8) +
    (min(s[6], s[7]) <<  4) +
    (max(s[6], s[7]) <<  0);
}

unordered_map<long, long> memo{};

// cost of transitioning to a fully-sorted state (all in their respective rooms)
long solve(state& s)
{
  long k = key(s);

  if (k == 0x789ABCDE) return 0;

  if (memo.count(k)) return memo[k];

  long best = 1'000'000;

  for (long i = 0; i < 8; i++) {
    for (long target = 0; target <= 0xE; target++) {
      long c = cost(s, i, target);
      if (c < 0) continue; // negative means move is not valid

      long tmp = s[i];
      s[i] = target;
      best = min(best, solve(s) + c);
      s[i] = tmp;
    }
  }

  memo[k] = best;
  return best;
}

// valid positions
// 01·2·3·4·56
//   7 9 b d
//   8 a c e

int main()
{
  // example:
  // #############
  // #...........#
  // ###B#C#B#D###
  //   #A#D#C#A#
  //   #########
  [[maybe_unused]] state example{0x8, 0xE, 0x7, 0xB, 0x9, 0xC, 0xA, 0xD};

  // input:
  // #############
  // #...........#
  // ###D#A#C#D###
  //   #B#C#B#A#
  //   #########
  state input{0x9, 0xE, 0x8, 0xC, 0xA, 0xB, 0x7, 0xD};

  cout << "Part 1: " << solve(input) << endl;
}
