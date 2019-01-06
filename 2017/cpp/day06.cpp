#include <iostream>
#include <algorithm>
#include <iterator>
#include <array>
using namespace std;

constexpr int N = 4;
using bank_array = array<int, N>;

void print_banks(const bank_array& banks) {
  copy(begin(banks), end(banks), ostream_iterator<int>(cout, " "));
  cout << "\n";
}

int part1(const bank_array& banks) {
  int cycles = 0;
  // TODO: not implemented
  return cycles;
}

int main() {
  bank_array banks {0, 2, 7, 0};
  cout << part1(banks) << "\n";
}
