#include <iostream>
#include <vector>
using namespace std;

int main() {
  vector<int> jumps, jumps2;
  int n;
  while (cin >> n) {
    jumps.push_back(n);
    jumps2.push_back(n);
  }

  int steps = 0;
  int idx = 0;
  while (idx >= 0 && idx < int(jumps.size())) {
    int jump = jumps[idx];
    jumps[idx] += 1;
    idx += jump;
    steps++;
  }

  cout << "Part 1: " << steps << '\n';

  steps = 0;
  idx = 0;
  while (idx >= 0 && idx < int(jumps2.size())) {
    int jump = jumps2[idx];
    if (jump >= 3) {
      jumps2[idx] -= 1;
    } else {
      jumps2[idx] += 1;
    }
    idx += jump;
    steps++;
  }

  cout << "Part 2: " << steps << '\n';
}
