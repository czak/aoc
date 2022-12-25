#include "stdafx.h"

istringstream example{R"(>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
)"};

using Row = bitset<9>;

struct Block
{
  array<Row, 4> rows;

  Row& operator[](size_t i) { return rows[i]; }
  Block operator<<(int n)
  {
    return Block{{rows[0] << n, rows[1] << n, rows[2] << n, rows[3] << n}};
  }
  Block operator>>(int n)
  {
    return Block{{rows[0] >> n, rows[1] >> n, rows[2] >> n, rows[3] >> n}};
  }
};

vector<Block> BLOCKS{
  {
    0b000111100,
    0b000000000,
    0b000000000,
    0b000000000,
  },
  {
    0b000010000,
    0b000111000,
    0b000010000,
    0b000000000,
  },
  {
    0b000111000,
    0b000001000,
    0b000001000,
    0b000000000,
  },
  {
    0b000100000,
    0b000100000,
    0b000100000,
    0b000100000,
  },
  {
    0b000110000,
    0b000110000,
    0b000000000,
    0b000000000,
  },
};

const Row EMPTY{0b100000001};

vector<Row> chamber{{0b111111111}};
vector<int> increments{};

void draw()
{
  for (auto it = chamber.rbegin(); it != chamber.rend(); it++) {
    for (bitset<9> mask{0b100000000}; mask != 0; mask >>= 1) {
      cout << ((*it & mask).any() ? '#' : '.');
    }
    cout << '\n';
  }
}

bool intersect(Block b, Row* c)
{
  return ((b[0] & c[0]) | (b[1] & c[1]) | (b[2] & c[2]) | (b[3] & c[3])) != 0;
}

void rest(Block b, Row* c)
{
  c[0] |= b[0];
  c[1] |= b[1];
  c[2] |= b[2];
  c[3] |= b[3];
}

void simulate(string wind)
{
  int woff = 0;
  int last_size = 1;

  for (int i = 0; i < 10'000; i++) {
    Block block = BLOCKS[i % BLOCKS.size()];

    // make room for new block
    size_t bottom = chamber.size() + 3;
    chamber.resize(chamber.size() + 7, EMPTY);

    while (true) {
      if (wind[woff] == '<') {
        if (!intersect(block << 1, &chamber[bottom])) block = block << 1;
      } else if (wind[woff] == '>') {
        if (!intersect(block >> 1, &chamber[bottom])) block = block >> 1;
      }
      woff = (woff + 1) % wind.size();

      if (intersect(block, &chamber[bottom - 1])) {
        rest(block, &chamber[bottom]);
        break;
      }

      bottom--;
    }

    // remove air from the end
    chamber.erase(find(chamber.begin(), chamber.end(), EMPTY), chamber.end());

    increments.push_back(chamber.size() - last_size);
    last_size = chamber.size();
  }
}

int part1()
{
  return accumulate(increments.begin(), increments.begin() + 2022, 0);
}

uint64_t part2()
{
  auto b = increments.begin();
  auto e = increments.end();

  // find cycle
  auto res = search(b + 2500, e, b + 2000, b + 2150);
  int cycle = distance(b + 2000, res);

  // find first
  int first = 0;
  for (first = 0; first < cycle; first++) {
    if (equal(b + first + 0 * cycle, b + first + 1 * cycle,
              b + first + 2 * cycle, b + first + 3 * cycle))
      break;
  }

  // find rest
  auto rest = (1000000000000 - first) % cycle;

  auto nfirst = accumulate(b, b + first, 0);
  auto ncycle = accumulate(b + first, b + first + cycle, 0);
  auto nrest = accumulate(b + first, b + first + rest, 0);

  return nfirst + (1000000000000 / cycle) * ncycle + nrest;
}

int main()
{
  string wind;
  getline(cin, wind);

  simulate(wind);

  cout << "Part 1: " << part1() << '\n';
  cout << "Part 2: " << part2() << '\n';
}
