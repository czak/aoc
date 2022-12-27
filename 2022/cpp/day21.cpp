#include "stdafx.h"
#include "utils.h"

istringstream example{R"(root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
)"};

using operation = tuple<char, string, string>;
using value = variant<long, operation>;

map<string, value> ops{};

void parse(istream& input)
{
  string s{istreambuf_iterator{input}, {}};
  ops = parse_to<map<string, value>>(
    s, regex{"(\\w{4}): (?:(\\d+)|(\\w{4}) ([+\\-*/]) (\\w{4}))"},
    [](auto sm) {
      if (sm[2].matched) return pair<string, value>{sm[1], stoi(sm[2])};
      return pair<string, value>{
        sm[1], tuple<char, string, string>{sm.str(4)[0], sm[3], sm[5]}
      };
    }
  );
}

long part1(string key)
{
  value val = ops[key];
  if (holds_alternative<long>(val)) return get<long>(val);

  auto [op, lhs, rhs] = get<operation>(val);

  switch (op) {
    case '+':
      return part1(lhs) + part1(rhs);
    case '-':
      return part1(lhs) - part1(rhs);
    case '*':
      return part1(lhs) * part1(rhs);
    case '/':
      return part1(lhs) / part1(rhs);
  }

  return -1;
}

int main()
{
  parse(cin);

  cout << "Part 1: " << part1("root") << '\n';
}
