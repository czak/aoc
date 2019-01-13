#include <iostream>
#include <set>
#include <unordered_set>
#include <unordered_map>
#include <regex>
using namespace std;

static const regex re {R"(\d+)"};

using conn_set = unordered_set<int>;
using conn_map = unordered_map<int, conn_set>;

conn_map read_input() {
  conn_map connections;
  string line;
  smatch sm;
  while (getline(cin, line)) {
    int m = -1;
    unordered_set<int> current;
    while (regex_search(line, sm, re)) {
      int n = stoi(sm.str());
      if (m == -1) {
        m = n;
      } else {
        current.insert(n);
      }
      line = sm.suffix();
    }
    connections.emplace(m, current);
  }
  return connections;
}

conn_map connections;

void merge(conn_set& s, const conn_set& other) {
  for (auto& n: other) {
    s.insert(n);
  }
}

conn_set neighbors(int n, conn_set& seen) {
  seen.insert(n);
  conn_set s {n};
  for (auto& i: connections.at(n)) {
    if (seen.count(i)) continue;
    merge(s, neighbors(i, seen));
  }
  return s;
}

struct set_hash {
  size_t operator()(const conn_set& s) const {
    size_t h = 0;
    for (auto& n: s) {
      h ^= std::hash<int>{}(n);
    }
    return h;
  }
};

int main() {
  connections = read_input();

  conn_set seen {0};
  conn_set s = neighbors(0, seen);
  cout << "Part 1: " << s.size() << '\n';

  unordered_set<conn_set, set_hash> groups {};
  for (int i=0; i<=1999; i++) {
    conn_set seen {i};
    conn_set s = neighbors(i, seen);
    groups.insert(s);
  }
  cout << "Part 2: " << groups.size() << '\n';
}
