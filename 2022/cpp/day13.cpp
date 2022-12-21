#include "stdafx.h"

istringstream example{R"([1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
)"};

bool compare(string sa, string sb)
{
  list<char> bufa{sa.begin(), sa.end()};
  list<char> bufb{sb.begin(), sb.end()};
  auto a = bufa.begin();
  auto b = bufb.begin();

  // clang-format off
  while (a != bufa.end() && b != bufb.end()) {
    if (*a == *b) { a++; b++; continue; }

    // compare digits
    if (isxdigit(*a) && isxdigit(*b)) return *a < *b;

    // one list ending
    if (*a == ']') return true;
    if (*b == ']') return false;

    // one list starting
    if (*a == '[') { bufb.insert(next(b, 1), ']'); a++; continue; }
    if (*b == '[') { bufa.insert(next(a, 1), ']'); b++; continue; }
  }
  // clang-format on

  return true;
}

vector<string> parse(istream& in)
{
  vector<string> v;
  regex r{"10"};
  string a, b, c;
  while (getline(in, a) && getline(in, b)) {
    v.push_back(regex_replace(a, r, "a"));
    v.push_back(regex_replace(b, r, "a"));
    getline(in, c);
  }
  return v;
}

int main()
{
  vector<string> lines = parse(cin);

  int sum = 0;
  for (size_t i = 0; i < lines.size(); i += 2) {
    if (compare(lines[i], lines[i + 1])) sum += i / 2 + 1;
  }

  cout << "Part 1: " << sum << '\n';

  lines.push_back("[[2]]");
  lines.push_back("[[6]]");

  sort(lines.begin(), lines.end(), compare);

  int i1 = find(lines.begin(), lines.end(), "[[2]]") - lines.begin() + 1;
  int i2 = find(lines.begin(), lines.end(), "[[6]]") - lines.begin() + 1;

  cout << "Part 2: " << i1 * i2 << '\n';
}
