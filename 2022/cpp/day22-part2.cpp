#include "stdafx.h"

istringstream example{R"(        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5)"};

map<int, pair<long, long>> directions{
  {0, {1, 0} },
  {1, {0, 1} },
  {2, {-1, 0}},
  {3, {0, -1}},
};

pair<int, int> side_coords[6]{
  {2, 0},
  {0, 1},
  {1, 1},
  {2, 1},
  {2, 2},
  {3, 2},
};

vector<string> board;
vector<string> cube[6];
string path;

int width, height;
int dim; // dimension = width of a cube side

void parse(istream& input)
{
  // board
  string line;
  while (getline(input, line)) {
    if (line.empty()) break;
    board.push_back(line);
  }

  // path
  getline(input, path);

  // make a rectangle
  width = max_element(board.begin(), board.end(), [](string a, string b) {
            return a.size() < b.size();
          })->size();
  for (string& l : board) {
    l.resize(width, ' ');
  }
  height = board.size();

  // split sides
  dim = width / 4;
  for (int side = 0; side < 6; side++) {
    auto [sx, sy] = side_coords[side];
    for (int y = sy * dim; y < (sy + 1) * dim; y++) {
      cube[side].push_back(board[y].substr(sx * dim, dim));
    }
  }
}

/*
        0000
        0000
        0000
        0000
111122223333
111122223333
111122223333
111122223333
        44445555
        44445555
        44445555
        44445555
*/

// clang-format off
void warp(int& s, int& x, int& y, int& o)
{
  // original x and y
  int ox = x;
  int oy = y;
  int end = dim - 1;

  if (o == 0) { // going right
    switch (s) {
      case 0: s = 5; x = end; y = end - oy; o = 2; break;
      case 1: s = 2; x = 0; /* y unchanged */ /* o unchanged */ break;
      case 2: s = 3; x = 0; /* y unchanged */ /* o unchanged */ break;
      case 3: s = 5; x = end - oy; y = 0; o = 1; break;
      case 4: s = 5; x = 0; /* y unchanged */ /* o unchanged */ break;
      case 5: s = 0; x = end; y = end - oy; o = 2; break;
    }
  } else if (o == 1) { // going down
    switch (s) {
      case 0: s = 3; /* x unchanged */ y = 0; /* o unchanged */ break;
      case 1: s = 4; x = end - ox; y = end; o = 3; break;
      case 2: s = 4; x = 0; y = ox; o = 0; break;
      case 3: s = 4; /* x unchanged */ y = 0; /* o unchanged */ break;
      case 4: s = 1; x = end - ox; y = end; o = 3; break;
      case 5: s = 1; x = 0; y = ox; o = 0; break;
    }
  } else if (o == 2) { // going left
    switch (s) {
      case 0: s = 2; x = oy; y = 0; o = 1; break;
      case 1: s = 5; x = end - oy; y = end; o = 3; break;
      case 2: s = 1; x = end; /* y unchanged */ /* o unchanged */ break;
      case 3: s = 2; x = end; /* y unchanged */ /* o unchanged */ break;
      case 4: s = 3; x = end - oy; y = end; o = 3; break;
      case 5: s = 4; x = end; /* y unchanged */ /* o unchanged */ break;
    }
  } else if (o == 3) { // going up
    switch(s) {
      case 0: s = 1; x = end - ox; y = 0; o = 1; break;
      case 1: s = 0; x = end - ox; y = 0; o = 1; break;
      case 2: s = 0; x = 0; y = ox; o = 0; break;
      case 3: s = 0; /* x unchanged */ y = end; /* o unchanged */ break;
      case 4: s = 3; /* x unchanged */ y = end; /* o unchanged */ break;
      case 5: s = 3; x = end; y = end - ox; o = 2; break;
    }
  }
}
// clang-format on

void move(int& s, int& x, int& y, int& o)
{
  int ns = s;
  int nx = x + directions[o].first;
  int ny = y + directions[o].second;
  int no = o;

  if (nx == -1 || ny == -1 || nx == dim || ny == dim) {
    warp(ns, nx, ny, no);
  }

  // only update if landed on empty
  if (cube[ns][ny][nx] == '.') {
    s = ns;
    x = nx;
    y = ny;
    o = no;
    cout << "-> " << s << ' ' << x << ',' << y << " >" << o << '\n';
  } else {
    cout << "-- " << s << ' ' << x << ',' << y << " >" << o << '\n';
  }
}

void rotate(int& o, char dir)
{
  if (dir == 'R') o = (o + 1) % 4;
  else o = (o + 3) % 4;
}

long part2()
{
  int s = 0, x = 0, y = 0, o = 0; // s = side, o = orientation

  int steps = 0;
  for (auto it = path.begin(); it != path.end(); it++) {
    char c = *it;
    if (c == 'R' || c == 'L') {
      for (int n = 0; n < steps; n++) {
        move(s, x, y, o);
      }
      steps = 0;
      rotate(o, c);
    } else {
      int n = c - '0';
      steps = steps * 10 + n;
    }
  }

  // last move
  for (int n = 0; n < steps; n++) {
    move(s, x, y, o);
  }

  // place on board
  x += side_coords[s].first * dim;
  y += side_coords[s].second * dim;

  return 1000 * (y + 1) + 4 * (x + 1) + o;
}

int main()
{
  parse(example);

  cout << "Part 2: " << part2() << '\n';
}
