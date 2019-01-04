#include <iostream>
#include <cmath>

enum direction { right, up, left, down };

int part1(int N) {
  // starting position
  int x = 0, y = 0;
  int n = 1;

  for (auto i = 0; n < N; i++) {
    auto stride = (i + 2) / 2;
    auto d = direction(i % 4);

    for (int j = 0; j < stride && n < N; j++, n++) {
      switch(d) {
        case right: x++; break;
        case up:    y--; break;
        case left:  x--; break;
        case down:  y++; break;
      }
    }
  }

  return std::abs(x) + std::abs(y);
}

int part2(int N) {
  const int SIZE = 20;
  int grid[SIZE][SIZE] {};

  int x = SIZE/2, y = SIZE/2, n = 1;
  grid[y][x] = n;

  for (auto i = 0; n <= N; i++) {
    auto stride = (i + 2) / 2;
    auto d = direction(i % 4);

    for (int j = 0; j < stride && n <= N; j++) {
      switch(d) {
        case right: x++; break;
        case up:    y--; break;
        case left:  x--; break;
        case down:  y++; break;
      }

      n = grid[y-1][x-1] + grid[y-1][x] + grid[y-1][x+1] +
          grid[y-0][x-1] +                grid[y-0][x+1] +
          grid[y+1][x-1] + grid[y+1][x] + grid[y+1][x+1];
      grid[y][x] = n;
    }
  }

  return n;
}

int main() {
  int N;
  std::cin >> N;

  std::cout << "Part 1: " << part1(N) << "\n";
  std::cout << "Part 2: " << part2(N) << "\n";
}
