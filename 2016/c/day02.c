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

  char c;
  while ((c = fgetc(f)) != EOF) {
    printf("%c\n", c);
  }
}
