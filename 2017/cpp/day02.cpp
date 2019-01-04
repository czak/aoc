#include <iostream>
#include <sstream>
#include <limits>
#include <vector>
#include <iterator>
using namespace std;

int part1(vector<string> lines) {
  int total = 0;

  for (auto line : lines) {
    istringstream is(line);
    int max = numeric_limits<int>::min();
    int min = numeric_limits<int>::max();
    for (int i; is >> i;) {
      if (i > max) {
        max = i;
      }
      if (i < min) {
        min = i;
      }
    }
    total += max - min;
  }

  return total;
}

int part2(vector<string> lines) {
  vector<vector<int>> data;

  for (auto line : lines) {
    istringstream is(line);
    data.push_back(vector<int>((istream_iterator<int>(is)), istream_iterator<int>()));
  }

  int total = 0;
  for (auto v : data) {
    for (auto a : v) {
      for (auto b : v) {
        if (a != b && a % b == 0) {
          total += a / b;
        }
      }
    }
  }

  return total;
}

int main() {
  vector<string> lines;
  for (string line; getline(cin, line);) {
    lines.push_back(line);
  }

  cout << "Part 1: " << part1(lines) << "\n";
  cout << "Part 2: " << part2(lines) << "\n";
}
