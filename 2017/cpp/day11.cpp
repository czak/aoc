#include <iostream>
#include <sstream>
#include <unordered_map>
using namespace std;

struct point { int x, y; };

point& operator+=(point& self, const point& other) {
  self.x += other.x;
  self.y += other.y;
  return self;
}

int dist(point& p) {
  if ((p.x > 0 && p.y > 0) || (p.x < 0 && p.y < 0)) {
    return abs(p.x) + abs(p.y);
  } else {
    return max(abs(p.x), abs(p.y));
  }
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
  int maxdist = 0;
  string dir;
  while (getline(is, dir, ',')) {
    p += directions[dir];
    if (dist(p) > maxdist) maxdist = dist(p);
  }

  cout << "Part 1: " << dist(p) << '\n';
  cout << "Part 2: " << maxdist << '\n';
}
