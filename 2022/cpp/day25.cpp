#include "stdafx.h"

istringstream example{R"(1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
)"};

unordered_map<char, long> digits{
  {'2', 2 },
  {'1', 1 },
  {'0', 0 },
  {'-', -1},
  {'=', -2},
};

long todecimal(string snafu)
{
  long n = 0;
  for (char c : snafu) {
    n = n * 5 + digits[c];
  }
  return n;
}

string tosnafu(long decimal)
{
  string snafu;

  while (decimal > 0) {
    long rest = decimal % 5;

    switch (rest) {
      case 0:
      case 1:
      case 2:
        snafu.insert(0, 1, '0' + rest);
        break;
      case 3:
        snafu.insert(0, 1, '=');
        decimal += 2;
        break;
      case 4:
        snafu.insert(0, 1, '-');
        decimal += 1;
        break;
    }

    decimal /= 5;
  }

  return snafu;
}

int main()
{
  string line;
  long sum = 0;

  while (getline(cin, line)) {
    sum += todecimal(line);
  }

  cout << "Part 1: " << tosnafu(sum) << '\n';
}
