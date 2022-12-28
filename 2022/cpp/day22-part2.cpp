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

void warp(int& s, int& x, int& y, int& o)
{
  if (o == 0) {
    // going right
    switch (s) {
      case 0:
      {
        s = 5;
        x = dim - 1;
        y = dim - y;
        o = 2;
        break;
      }
      case 1:
      {
        s = 2;
        x = 0;
        // y unchanged
        // o unchanged
        break;
      }
      case 2:
      {
        s = 3;
        x = 0;
        // y unchanged
        // o unchanged
        break;
      }
      case 3:
      {
        s = 5;
        x = dim - y;
        y = 0;
        o = 1;
      }
    }
  }
}

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

  return 1000 * (y + 1) + 4 * (x + 1) + o;
}

int main()
{
  parse(example);

  dbg(cube);

  // cout << "Part 1: " << part1() << '\n';
}
