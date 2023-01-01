#include "stdafx.h"

// vector<int> MEM(122'000'000); // ~105^4
unordered_map<int, int> MEM;

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

int solve(int a1, int a2, int b1, int b2, int c1, int c2, int d1, int d2)
{
  if (a1 > a2) swap(a1, a2);
  if (b1 > b2) swap(b1, b2);
  if (c1 > c2) swap(c1, c2);
  if (d1 > d2) swap(d1, d2);

  // all in their respective rooms
  if ((a1 == 0x7 && a2 == 0x8) && (b1 == 0x9 && b2 == 0xA) && (c1 == 0xB && c2 == 0xC) && (d1 == 0xD && d2 == 0xe))
    return 0;

  return 0;
}

int main()
{
  solve(9, 8, 0, 0, 0, 0, 0, 0);
}
