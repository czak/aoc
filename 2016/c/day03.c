#include <stdio.h>

int main() {
  FILE *f = fopen("../in/day03.in", "r");

  // Part 1
  int count = 0;

  int a, b, c;
  while (fscanf(f, "%d %d %d", &a, &b, &c) != EOF) {
    if (a + b > c && a + c > b && b + c > a)
      count += 1;
  }

  printf("Part 1: %d\n", count);

  // Part 2
  fseek(f, 0, SEEK_SET);
  count = 0;

  int a1, a2, a3, b1, b2, b3, c1, c2, c3;
  while (fscanf(f, "%d %d %d %d %d %d %d %d %d", &a1, &b1, &c1, &a2, &b2, &c2,
                &a3, &b3, &c3) != EOF) {
    if (a1 + a2 > a3 && a1 + a3 > a2 && a2 + a3 > a1)
      count += 1;
    if (b1 + b2 > b3 && b1 + b3 > b2 && b2 + b3 > b1)
      count += 1;
    if (c1 + c2 > c3 && c1 + c3 > c2 && c2 + c3 > c1)
      count += 1;
  }

  printf("Part 2: %d\n", count);
}
