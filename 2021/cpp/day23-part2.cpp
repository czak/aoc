#include "stdafx.h"

using state = array<long, 16>;
using path = pair<vector<long>, long>; // path does NOT INCLUDE 'from' but INCLUDES 'to'

// 01·2·3·4·56
//   7 b f 3
//   8 c 0 4
//   9 d 1 5
//   a e 2 6

// see day23-precalc.cpp
const array<array<optional<path>, 0x16+1>, 0x16+1> paths{
  {
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{1, 7}, 3}, path{{1, 7, 8}, 4}, path{{1, 7, 8, 9}, 5}, path{{1, 7, 8, 9, 10}, 6}, path{{1, 2, 11}, 5}, path{{1, 2, 11, 12}, 6}, path{{1, 2, 11, 12, 13}, 7}, path{{1, 2, 11, 12, 13, 14}, 8}, path{{1, 2, 3, 15}, 7}, path{{1, 2, 3, 15, 16}, 8}, path{{1, 2, 3, 15, 16, 17}, 9}, path{{1, 2, 3, 15, 16, 17, 18}, 10}, path{{1, 2, 3, 4, 19}, 9}, path{{1, 2, 3, 4, 19, 20}, 10}, path{{1, 2, 3, 4, 19, 20, 21}, 11}, path{{1, 2, 3, 4, 19, 20, 21, 22}, 12} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{7}, 2}, path{{7, 8}, 3}, path{{7, 8, 9}, 4}, path{{7, 8, 9, 10}, 5}, path{{2, 11}, 4}, path{{2, 11, 12}, 5}, path{{2, 11, 12, 13}, 6}, path{{2, 11, 12, 13, 14}, 7}, path{{2, 3, 15}, 6}, path{{2, 3, 15, 16}, 7}, path{{2, 3, 15, 16, 17}, 8}, path{{2, 3, 15, 16, 17, 18}, 9}, path{{2, 3, 4, 19}, 8}, path{{2, 3, 4, 19, 20}, 9}, path{{2, 3, 4, 19, 20, 21}, 10}, path{{2, 3, 4, 19, 20, 21, 22}, 11} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{7}, 2}, path{{7, 8}, 3}, path{{7, 8, 9}, 4}, path{{7, 8, 9, 10}, 5}, path{{11}, 2}, path{{11, 12}, 3}, path{{11, 12, 13}, 4}, path{{11, 12, 13, 14}, 5}, path{{3, 15}, 4}, path{{3, 15, 16}, 5}, path{{3, 15, 16, 17}, 6}, path{{3, 15, 16, 17, 18}, 7}, path{{3, 4, 19}, 6}, path{{3, 4, 19, 20}, 7}, path{{3, 4, 19, 20, 21}, 8}, path{{3, 4, 19, 20, 21, 22}, 9} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{2, 7}, 4}, path{{2, 7, 8}, 5}, path{{2, 7, 8, 9}, 6}, path{{2, 7, 8, 9, 10}, 7}, path{{11}, 2}, path{{11, 12}, 3}, path{{11, 12, 13}, 4}, path{{11, 12, 13, 14}, 5}, path{{15}, 2}, path{{15, 16}, 3}, path{{15, 16, 17}, 4}, path{{15, 16, 17, 18}, 5}, path{{4, 19}, 4}, path{{4, 19, 20}, 5}, path{{4, 19, 20, 21}, 6}, path{{4, 19, 20, 21, 22}, 7} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{3, 2, 7}, 6}, path{{3, 2, 7, 8}, 7}, path{{3, 2, 7, 8, 9}, 8}, path{{3, 2, 7, 8, 9, 10}, 9}, path{{3, 11}, 4}, path{{3, 11, 12}, 5}, path{{3, 11, 12, 13}, 6}, path{{3, 11, 12, 13, 14}, 7}, path{{15}, 2}, path{{15, 16}, 3}, path{{15, 16, 17}, 4}, path{{15, 16, 17, 18}, 5}, path{{19}, 2}, path{{19, 20}, 3}, path{{19, 20, 21}, 4}, path{{19, 20, 21, 22}, 5} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{4, 3, 2, 7}, 8}, path{{4, 3, 2, 7, 8}, 9}, path{{4, 3, 2, 7, 8, 9}, 10}, path{{4, 3, 2, 7, 8, 9, 10}, 11}, path{{4, 3, 11}, 6}, path{{4, 3, 11, 12}, 7}, path{{4, 3, 11, 12, 13}, 8}, path{{4, 3, 11, 12, 13, 14}, 9}, path{{4, 15}, 4}, path{{4, 15, 16}, 5}, path{{4, 15, 16, 17}, 6}, path{{4, 15, 16, 17, 18}, 7}, path{{19}, 2}, path{{19, 20}, 3}, path{{19, 20, 21}, 4}, path{{19, 20, 21, 22}, 5} },
    { nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, path{{5, 4, 3, 2, 7}, 9}, path{{5, 4, 3, 2, 7, 8}, 10}, path{{5, 4, 3, 2, 7, 8, 9}, 11}, path{{5, 4, 3, 2, 7, 8, 9, 10}, 12}, path{{5, 4, 3, 11}, 7}, path{{5, 4, 3, 11, 12}, 8}, path{{5, 4, 3, 11, 12, 13}, 9}, path{{5, 4, 3, 11, 12, 13, 14}, 10}, path{{5, 4, 15}, 5}, path{{5, 4, 15, 16}, 6}, path{{5, 4, 15, 16, 17}, 7}, path{{5, 4, 15, 16, 17, 18}, 8}, path{{5, 19}, 3}, path{{5, 19, 20}, 4}, path{{5, 19, 20, 21}, 5}, path{{5, 19, 20, 21, 22}, 6} },
    { path{{1, 0}, 3}, path{{1}, 2}, path{{2}, 2}, path{{2, 3}, 4}, path{{2, 3, 4}, 6}, path{{2, 3, 4, 5}, 8}, path{{2, 3, 4, 5, 6}, 9}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{7, 1, 0}, 4}, path{{7, 1}, 3}, path{{7, 2}, 3}, path{{7, 2, 3}, 5}, path{{7, 2, 3, 4}, 7}, path{{7, 2, 3, 4, 5}, 9}, path{{7, 2, 3, 4, 5, 6}, 10}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{8, 7, 1, 0}, 5}, path{{8, 7, 1}, 4}, path{{8, 7, 2}, 4}, path{{8, 7, 2, 3}, 6}, path{{8, 7, 2, 3, 4}, 8}, path{{8, 7, 2, 3, 4, 5}, 10}, path{{8, 7, 2, 3, 4, 5, 6}, 11}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{9, 8, 7, 1, 0}, 6}, path{{9, 8, 7, 1}, 5}, path{{9, 8, 7, 2}, 5}, path{{9, 8, 7, 2, 3}, 7}, path{{9, 8, 7, 2, 3, 4}, 9}, path{{9, 8, 7, 2, 3, 4, 5}, 11}, path{{9, 8, 7, 2, 3, 4, 5, 6}, 12}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{2, 1, 0}, 5}, path{{2, 1}, 4}, path{{2}, 2}, path{{3}, 2}, path{{3, 4}, 4}, path{{3, 4, 5}, 6}, path{{3, 4, 5, 6}, 7}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{11, 2, 1, 0}, 6}, path{{11, 2, 1}, 5}, path{{11, 2}, 3}, path{{11, 3}, 3}, path{{11, 3, 4}, 5}, path{{11, 3, 4, 5}, 7}, path{{11, 3, 4, 5, 6}, 8}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{12, 11, 2, 1, 0}, 7}, path{{12, 11, 2, 1}, 6}, path{{12, 11, 2}, 4}, path{{12, 11, 3}, 4}, path{{12, 11, 3, 4}, 6}, path{{12, 11, 3, 4, 5}, 8}, path{{12, 11, 3, 4, 5, 6}, 9}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{13, 12, 11, 2, 1, 0}, 8}, path{{13, 12, 11, 2, 1}, 7}, path{{13, 12, 11, 2}, 5}, path{{13, 12, 11, 3}, 5}, path{{13, 12, 11, 3, 4}, 7}, path{{13, 12, 11, 3, 4, 5}, 9}, path{{13, 12, 11, 3, 4, 5, 6}, 10}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{3, 2, 1, 0}, 7}, path{{3, 2, 1}, 6}, path{{3, 2}, 4}, path{{3}, 2}, path{{4}, 2}, path{{4, 5}, 4}, path{{4, 5, 6}, 5}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{15, 3, 2, 1, 0}, 8}, path{{15, 3, 2, 1}, 7}, path{{15, 3, 2}, 5}, path{{15, 3}, 3}, path{{15, 4}, 3}, path{{15, 4, 5}, 5}, path{{15, 4, 5, 6}, 6}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{16, 15, 3, 2, 1, 0}, 9}, path{{16, 15, 3, 2, 1}, 8}, path{{16, 15, 3, 2}, 6}, path{{16, 15, 3}, 4}, path{{16, 15, 4}, 4}, path{{16, 15, 4, 5}, 6}, path{{16, 15, 4, 5, 6}, 7}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{17, 16, 15, 3, 2, 1, 0}, 10}, path{{17, 16, 15, 3, 2, 1}, 9}, path{{17, 16, 15, 3, 2}, 7}, path{{17, 16, 15, 3}, 5}, path{{17, 16, 15, 4}, 5}, path{{17, 16, 15, 4, 5}, 7}, path{{17, 16, 15, 4, 5, 6}, 8}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{4, 3, 2, 1, 0}, 9}, path{{4, 3, 2, 1}, 8}, path{{4, 3, 2}, 6}, path{{4, 3}, 4}, path{{4}, 2}, path{{5}, 2}, path{{5, 6}, 3}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{19, 4, 3, 2, 1, 0}, 10}, path{{19, 4, 3, 2, 1}, 9}, path{{19, 4, 3, 2}, 7}, path{{19, 4, 3}, 5}, path{{19, 4}, 3}, path{{19, 5}, 3}, path{{19, 5, 6}, 4}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{20, 19, 4, 3, 2, 1, 0}, 11}, path{{20, 19, 4, 3, 2, 1}, 10}, path{{20, 19, 4, 3, 2}, 8}, path{{20, 19, 4, 3}, 6}, path{{20, 19, 4}, 4}, path{{20, 19, 5}, 4}, path{{20, 19, 5, 6}, 5}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
    { path{{21, 20, 19, 4, 3, 2, 1, 0}, 12}, path{{21, 20, 19, 4, 3, 2, 1}, 11}, path{{21, 20, 19, 4, 3, 2}, 9}, path{{21, 20, 19, 4, 3}, 7}, path{{21, 20, 19, 4}, 5}, path{{21, 20, 19, 5}, 5}, path{{21, 20, 19, 5, 6}, 6}, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt, nullopt },
  },
};

const state multipliers{1, 1, 1, 1, 10, 10, 10, 10, 100, 100, 100, 100, 1000, 1000, 1000, 1000};

// cost of moving i-th amphipod in s to 'to' location
// - includes multiplier
long cost(const state& s, long i, long to)
{
  const long from = s[i];
  // which group 'i' belongs to: 0 == A, 1 == B, 2 == C, 3 == D
  const long group = i / 4;
  // loc of first chamber in valid room (e.g. 0x07 for A, 0x0B for B, ...)
  const long room_base = group * 4 + 0x07;
  // for convenience locations of room chambers (r0 is first chamber from the corridor, r3 is deepest)
  const long r0 = room_base, r1 = room_base + 1, r2 = room_base + 2, r3 = room_base + 3;

  // Invalid moves
  // - no path
  const auto& p = paths[from][to];
  if (!p) return -1;

  // - moving from the corridor into a wrong room
  if (from <= 0x6 && (to < r0 || to > r3))
    return -1;

  bool occupied[0x16 + 1]{};
  bool mine[0x16 + 1]{};
  for (long j = 0; j < 16; j++) {
    occupied[s[j]] = true;
    mine[s[j]] = (j / 4 == group); // mine[x] == true if x is occupied by one of my own group
  }

  // moving into the right room but into an invalid chamber
  // - not all the way in
  // - or the room is occupied by an incompatible amphipod
  if (from <= 0x06) {
    if (to == r2 && !mine[r3])
      return -1;
    if (to == r1 && (!mine[r2] || !mine[r3]))
      return -1;
    if (to == r0 && (!mine[r1] || !mine[r2] || !mine[r3]))
      return -1;
  }

  // - trying to move out of a "locked correct" state in correct room
  if (to <= 0x06) {
    if (from == r3) return -1;
    if (from == r2 && (mine[r3]))
      return -1;
    if (from == r1 && (mine[r2] && mine[r3]))
      return -1;
    if (from == r0 && (mine[r1] && mine[r2] && mine[r3]))
      return -1;
  }

  // --- from here on we're dealing with valid moves, but the path can be blocked ---

  for (auto& step : p->first) {
    if (occupied[step]) return -1;
  }

  return p->second * multipliers[i];
}

const state solved{
  0x07, 0x08, 0x09, 0x0A,
  0x0B, 0x0C, 0x0D, 0x0E,
  0x0F, 0x10, 0x11, 0x12,
  0x13, 0x14, 0x15, 0x16,
};

state normalized(state s)
{
  sort(s.begin() + 0, s.begin() + 4);
  sort(s.begin() + 4, s.begin() + 8);
  sort(s.begin() + 8, s.begin() + 12);
  sort(s.begin() + 12, s.begin() + 16);
  return s;
}

map<state, long> memo{};

// cost of transitioning to a fully-sorted state (all in their respective rooms)
long solve(state& s, int level=0)
{
  state k = normalized(s);

  if (k == solved) return 0;

  if (memo.count(k)) return memo[k];

  long best = 1'000'000;

  for (long i = 0; i < 16; i++) {
    for (long target = 0; target <= 0x16; target++) {
      long c = cost(s, i, target);
      if (c < 0) continue; // negative means move is not valid

      long tmp = s[i];
      s[i] = target;
      best = min(best, solve(s, level+1) + c);
      s[i] = tmp;
    }
  }

  memo[k] = best;
  return best;
}

// valid positions
// 01·2·3·4·56
//   7 b f 3
//   8 c 0 4
//   9 d 1 5
//   a e 2 6

int main()
{
  // example:
  // #############
  // #...........#
  // ###B#C#B#D###
  //   #D#C#B#A#
  //   #D#B#A#C#
  //   #A#D#C#A#
  //   #########
  [[maybe_unused]] state example{
    0x0A, 0x11, 0x14, 0x16,
    0x07, 0x0D, 0x0F, 0x10,
    0x0B, 0x0C, 0x12, 0x15,
    0x08, 0x09, 0x0E, 0x13,
  };

  cout << "Example: " << solve(example) << endl;

  // input:
  // #############
  // #...........#
  // ###D#A#C#D###
  //   #D#C#B#A#
  //   #D#B#A#C#
  //   #B#C#B#A#
  //   #########

  [[maybe_unused]] state input{
    0x0B, 0x11, 0x14, 0x16,
    0x0A, 0x0D, 0x10, 0x12,
    0x0C, 0x0E, 0x0F, 0x15,
    0x07, 0x08, 0x09, 0x13,
  };

  cout << "Part 2: " << solve(input) << endl;
}
