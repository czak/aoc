#include "stdafx.h"

const vector<vector<uint8_t>> BLOCKS{
  {
    0b00011110,
  },
  {
    0b00001000,
    0b00011100,
    0b00001000,
  },
  {
    0b00000100,
    0b00000100,
    0b00011100,
  },
  {
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
  },
  {
    0b00011000,
    0b00011000,
  },
};

vector<uint8_t> chamber{0b11111111}; // floor only

void draw()
{
  for (auto it = chamber.rbegin(); it != chamber.rend(); it++) {
    for (uint8_t mask = 64; mask > 0; mask /= 2) {
      cout << ((*it & mask) ? '#' : '.');
    }
    cout << '\n';
  }
}

void part1()
{
  for (int i = 0; i < 1; i++) {
    auto block = BLOCKS[i % BLOCKS.size()];
    chamber.resize(chamber.size() + 3 + block.size(), 0b00000000);
  }

  draw();
}

int main()
{
  part1();
}
