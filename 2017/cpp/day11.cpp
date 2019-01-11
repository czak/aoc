#include <utility>
#include <iostream>
#include <sstream>
#include <unordered_map>
using namespace std;

using point = pair<int, int>;

point& operator+=(point& self, const point& other) {
  self.first += other.first;
  self.second += other.second;
  return self;
}

int main() {
  string input;
  cin >> input;
  istringstream is {input};

  unordered_map<string, point> directions {
    {"n", {0, 1}},
    {"ne", {1, 0}},
    {"se", {1, -1}},
    {"s", {0, -1}},
    {"sw", {-1, 0}},
    {"nw", {-1, 1}},
  };

  point p {0, 0};

  string dir;
  while (getline(is, dir, ',')) {
    p += directions[dir];
  }

  cout << p.first << ',' << p.second << '\n';
}
