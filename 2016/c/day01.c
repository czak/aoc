#include <stdio.h>

int main() {
  FILE *f = fopen("../in/day01.in", "r");

  int x = 0, y = 0;
  enum { NORTH, EAST, SOUTH, WEST } dir = NORTH;

  char turn;
  int steps;

  while (fscanf(f, "%c%d", &turn, &steps) != EOF) {
    printf("%c, %d\n", turn, steps);

    if (turn == 'R')
      dir = (dir + 1) % 4;
    else if (turn == 'L')
      dir = (dir - 1) % 4;

    switch (dir) {
    case NORTH:
      y += steps;
      break;
    case EAST:
      x += steps;
      break;
    case SOUTH:
      y -= steps;
      break;
    case WEST:
      x -= steps;
      break;
    }

    fseek(f, 2, SEEK_CUR);
  }

  printf("%d, %d\n", x, y);
}
