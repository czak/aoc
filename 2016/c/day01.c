#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define CENTER 250

struct point {
  int x;
  int y;
};

const struct point DIRS[4] = {
    {0, 1},
    {1, 0},
    {0, -1},
    {-1, 0},
};

int main() {
  FILE *f = fopen("../in/day01.in", "r");

  struct point p = {0, 0};
  enum { NORTH, EAST, SOUTH, WEST } dir = NORTH;

  struct {
    int grid[2 * CENTER][2 * CENTER];
    struct point p;
    int found;
  } visited = {0};

  char turn;
  int steps;

  while (fscanf(f, "%c%d", &turn, &steps) != EOF) {
    if (turn == 'R')
      dir = (dir + 1) % 4;
    else if (turn == 'L')
      dir = (dir - 1) % 4;

    for (int i = 0; i < steps; i++) {
      p.x += DIRS[dir].x;
      p.y += DIRS[dir].y;

      if (!visited.found && visited.grid[CENTER + p.x][CENTER + p.y]) {
        visited.found = 1;
        visited.p.x = p.x;
        visited.p.y = p.y;
      }

      visited.grid[CENTER + p.x][CENTER + p.y] = 1;
    }

    fseek(f, 2, SEEK_CUR);
  }

  printf("Part 1: %d\n", abs(p.x) + abs(p.y));
  printf("Part 2: %d\n", abs(visited.p.x) + abs(visited.p.y));
}
