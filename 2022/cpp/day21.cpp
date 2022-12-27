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

long solve(string key)
{
  value val = ops[key];
  if (holds_alternative<long>(val)) return get<long>(val);

  auto [op, lhs, rhs] = get<operation>(val);

  switch (op) {
    case '+':
      return solve(lhs) + solve(rhs);
    case '-':
      return solve(lhs) - solve(rhs);
    case '*':
      return solve(lhs) * solve(rhs);
    case '/':
      return solve(lhs) / solve(rhs);
  }

  return -1;
}

int main()
{
  parse(cin);

  cout << "Part 1: " << solve("root") << '\n';

  // większe od 3266318702264
  // mniejsze od 3366318702264

  for (long i = 3296135382264; i < 3366318702264; i++) {
    ops["humn"] = i;

    long lvvf = solve("lvvf");
    long rqgq = solve("rqgq");

    if (lvvf == rqgq) {
      cout << "Part 2: " << i << '\n';
      break;
    }

    cout << abs(lvvf - rqgq) << '\r';
  }
}
