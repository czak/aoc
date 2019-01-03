#include <iostream>
using namespace std;

int part1(string s) {
  int total = 0;
  auto last = s.back();

  for (auto it = s.begin(); it != s.end(); it++) {
    if (*it == last) {
      total += (*it) - '0';
    }

    last = *it;
  }

  return total;
}

int part2(string s) {
  string s2 = s.substr(s.size() / 2) + s.substr(0, s.size() / 2);
  int total = 0;
  int len = s.size();

  for (int i = 0; i < len; i++) {
    if (s[i] == s2[i]) {
      total += s[i] - '0';
    }
  }

  return total;
}

int main() {
  string s;
  cin >> s;

  cout << "Part 1: " << part1(s) << "\n";
  cout << "Part 2: " << part2(s) << "\n";
}
