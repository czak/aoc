#include <string>
#include <iostream>
#include <utility>
using namespace std;

pair<int, int> score_and_clean(const string& s) {
  auto it = s.begin();

  int score = 0;
  int garbage = 0;

  int level = 0;
  bool in_garbage = false;

  while (it != s.end()) {
    if (in_garbage) {
      switch (*it) {
        case '>': in_garbage = false; break;
        case '!': it++; break;
        default: garbage++;
      }
    } else {
      switch (*it) {
        case '{': level++; break;
        case '}': score += level; level--; break;
        case '<': in_garbage = true; break;
      }
    }
    it++;
  }

  return {score, garbage};
}

int main() {
  string s;
  cin >> s;

  auto res = score_and_clean(s);
  cout << "Part 1: " << res.first << '\n';
  cout << "Part 2: " << res.second << '\n';
}
