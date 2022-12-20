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

struct node
{
  variant<int, vector<node>> data;
};

ostream& operator<<(ostream& out, const node& n)
{
  if (holds_alternative<int>(n.data)) out << get<int>(n.data);
  else dbg::pretty_print(out, get<vector<node>>(n.data));
  return out;
}

int main()
{
  string s;
  while (getline(example, s)) {
    cout << "|||" << s << "|||\n";
  }
  node l1{1};
  dbg(l1);
}
