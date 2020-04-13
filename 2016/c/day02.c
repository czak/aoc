#include <stdio.h>

void part1(FILE *f);
void part2(FILE *f);

int main() {
  FILE *f = fopen("../in/day02.in", "r");

  part1(f);
  part2(f);
}

void part1(FILE *f) {
  fseek(f, 0, SEEK_SET);

  // 1 2 3
  // 4 5 6
  // 7 8 9

  int code[5] = {0};
  int pos = 0;
  int n = 5;

  char c;
  while ((c = fgetc(f)) != EOF) {
    switch (c) {
    case '\n':
      code[pos] = n;
      pos += 1;
      break;
    case 'U':
      if (n > 3)
        n -= 3;
      break;
    case 'D':
      if (n < 7)
        n += 3;
      break;
    case 'L':
      if (n % 3 != 1)
        n -= 1;
      break;
    case 'R':
      if (n % 3 != 0)
        n += 1;
      break;
    }
  }

  printf("%d%d%d%d%d\n", code[0], code[1], code[2], code[3], code[4]);
}

void part2(FILE *f) {
  fseek(f, 0, SEEK_SET);

  //     1
  //   2 3 4
  // 5 6 7 8 9
  //   A B C
  //     D

  // clang-format off
  int keypad[7][7] = {
    { 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 },
    { 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0 },
    { 0x0, 0x0, 0x2, 0x3, 0x4, 0x0, 0x0 },
    { 0x0, 0x5, 0x6, 0x7, 0x8, 0x9, 0x0 },
    { 0x0, 0x0, 0xa, 0xb, 0xc, 0x0, 0x0 },
    { 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0x0 },
    { 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 },
  };
  // clang-format on

  struct {
    int x;
    int y;
  } cur = {1, 3};

  int code[5] = {};
  int i = 0;

  char c;
  while ((c = fgetc(f)) != EOF) {
    switch (c) {
    case '\n':
      code[i++] = keypad[cur.y][cur.x];
      break;

    case 'U':
      if (keypad[cur.y - 1][cur.x] != 0)
        cur.y -= 1;
      break;

    case 'D':
      if (keypad[cur.y + 1][cur.x] != 0)
        cur.y += 1;
      break;

    case 'L':
      if (keypad[cur.y][cur.x - 1] != 0)
        cur.x -= 1;
      break;

    case 'R':
      if (keypad[cur.y][cur.x + 1] != 0)
        cur.x += 1;
      break;
    }
  }

  printf("%X%X%X%X%X\n", code[0], code[1], code[2], code[3], code[4]);
}
