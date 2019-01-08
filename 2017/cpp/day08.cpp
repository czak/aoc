#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <regex>
#include <limits>
using namespace std;

using reg_map = unordered_map<string, int>;

regex re {"(\\w+) (inc|dec) (-?\\d+) if (\\w+) (\\S+) (-?\\d+)"};

// 0: 'c dec -10 if a >= 1'
// 1: 'c'
// 2: 'dec'
// 3: '-10'
// 4: 'a'
// 5: '>='
// 6: '1'

bool cond(reg_map& R, string r, string comp, string val) {
  int n = stoi(val);
  if (comp == "==") return (R[r] == n);
  if (comp == "!=") return (R[r] != n);
  if (comp == ">=") return (R[r] >= n);
  if (comp == "<=") return (R[r] <= n);
  if (comp == ">") return (R[r] > n);
  if (comp == "<") return (R[r] < n);
  return false;
}

void op(reg_map& R, string r, string o, string val) {
  int n = stoi(val);
  if (o == "inc") R[r] += n;
  if (o == "dec") R[r] -= n;
}

int main() {
  reg_map R {};

  int max_ever = numeric_limits<int>::min();

  string s;
  smatch m;
  while (getline(cin, s)) {
    if (!regex_match(s, m, re) || m.size() != 7) {
      cerr << "Failed parsing " << s << "\n";
      return 1;
    }

    if (cond(R, m[4].str(), m[5].str(), m[6].str())) {
      op(R, m[1].str(), m[2].str(), m[3].str());
      if (R[m[1].str()] > max_ever) {
        max_ever = R[m[1].str()];
      }
    }
  }

  int max = numeric_limits<int>::min();
  for (const auto& p: R) {
    if (p.second > max) {
      max = p.second;
    }
  }

  cout << "Part 1: " << max << '\n';
  cout << "Part 2: " << max_ever << '\n';
}
