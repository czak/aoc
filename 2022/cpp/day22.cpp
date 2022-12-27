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

vector<string> board;
string path;

int width, height;

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
}

void move(int& x, int& y, int o)
{
  int dx = directions[o].first;
  int dy = directions[o].second;

  int nx = x;
  int ny = y;

  bool settled = false;
  while (!settled) {
    nx += dx;
    ny += dy;

    if (nx == -1) {
      nx = width;
    } else if (ny == -1) {
      ny = height;
    } else if (nx == width) {
      nx = -1;
    } else if (ny == height) {
      ny = -1;
    } else if (board[ny][nx] == '.') {
      x = nx;
      y = ny;
      settled = true;
    } else if (board[ny][nx] == '#') {
      settled = true;
    }
  }
}

void rotate(int& o, char dir)
{
  if (dir == 'R') o = (o + 1) % 4;
  else o = (o + 3) % 4;
}

long part1()
{
  int x = 0, y = 0, o = 0; // o = orientation

  // locate start position in first row
  for (x = 0; x < int(board[0].size()); x++) {
    if (board[0][x] == '.') break;
  }

  int steps = 0;
  for (auto it = path.begin(); it != path.end(); it++) {
    char c = *it;
    if (c == 'R' || c == 'L') {
      for (int n = 0; n < steps; n++) {
        move(x, y, o);
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
    move(x, y, o);
  }

  return 1000 * (y + 1) + 4 * (x + 1) + o;
}

int main()
{
  parse(cin);

  cout << "Part 1: " << part1() << '\n';
}
