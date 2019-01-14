#include <iostream>
#include <array>
#include <vector>
#include <iomanip>
using namespace std;

#define N 256

void reverse(int* list, int current, int length) {
  int l = current;
  int r = (current + length - 1) % N;
  for (int i=0; i<length/2; i++) {
    int tmp = list[l];
    list[l] = list[r];
    list[r] = tmp;

    l++; if (l >= N) l = 0;
    r--; if (r < 0) r = N - 1;
  }
}

array<int, 16> knot_hash(string s) {
  int list[N];
  for (int i=0; i<N; i++) {
    list[i] = i;
  }

  vector<int> lengths;
  for (auto& c: s) {
    lengths.push_back(c);
  }

  lengths.push_back(17);
  lengths.push_back(31);
  lengths.push_back(73);
  lengths.push_back(47);
  lengths.push_back(23);

  int current = 0;
  int skip = 0;

  for (int i=0; i<64; i++) {
    for (auto& length: lengths) {
      reverse(list, current, length);
      current = (current + length + skip) % N;
      skip++;
    }
  }

  array<int, 16> res {};
  for (int i=0; i<16; i++) {
    int n = 0;
    for (int j=0; j<16; j++) {
      n ^= list[16*i+j];
    }
    res[i] = n;
  }

  return res;
}

using grid = array<array<int, 128>, 128>;

void fill_region(int row, int col, int num, grid& disk, grid& regions) {
  if (row < 0 || row >= 128 || col < 0 || col >= 128) return;
  if (!disk[row][col] || regions[row][col] > 0) return;
  regions[row][col] = num;
  fill_region(row-1, col, num, disk, regions);
  fill_region(row+1, col, num, disk, regions);
  fill_region(row, col-1, num, disk, regions);
  fill_region(row, col+1, num, disk, regions);
}

int main() {
  // string prefix = "flqrgnkx"; // example
  string prefix = "nbysizxe";
  int used = 0;

  grid disk;

  for (int row=0; row<128; row++) {
    auto hash = knot_hash(prefix + "-" + to_string(row));
    int bit = 0;
    for (auto n: hash) {
      for (int i=128; i>0; i/=2) {
        if (n & i) {
          cout << '#';
          disk[row][bit] = 1;
          used++;
        } else {
          cout << '.';
          disk[row][bit] = 0;
        }
        bit++;
      }
    }
    cout << '\n';
  }

  cout << "Part 1: " << used << '\n';

  grid regions;
  int current = 0;
  for (int row=0; row<128; row++) {
    for (int col=0; col<128; col++) {
      if (!disk[row][col] || regions[row][col] > 0) continue;
      fill_region(row, col, ++current, disk, regions);
    }
  }

  cout << "Part 2: " << current << '\n';
}
