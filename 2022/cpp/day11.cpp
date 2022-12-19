#include "stdafx.h"
#include "utils.h"

using namespace std;

extern const std::regex NUMBER_REGEX;
extern const std::regex MONKEY_REGEX;

istringstream example{R"(Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1)"};

struct Monkey {
  list<int> items;
  char operation;
  string operand;
  int test;
  size_t if_true;
  size_t if_false;
};

ostream &operator<<(ostream &out, const Monkey &m) {
  out << "{";
  // clang-format off
  dbg::pretty_print(out, m.items); out << ", ";
  dbg::pretty_print(out, m.operation); out << ", ";
  dbg::pretty_print(out, m.operand); out << ", ";
  dbg::pretty_print(out, m.test); out << ", ";
  dbg::pretty_print(out, m.if_true); out << ", ";
  dbg::pretty_print(out, m.if_false);
  // clang-format on
  out << "}";
  return out;
}

void dump(const vector<Monkey> &monkeys) {
  for (int i = 0; i < monkeys.size(); i++) {
    cout << "Monkey " << i << ": " << monkeys[i] << '\n';
  }
}

int main() {
  string data{istreambuf_iterator{example}, {}};
  auto monkeys = parse_to<vector<Monkey>>(data, MONKEY_REGEX, [](auto &sm) {
    return Monkey{
        parse_to<list<int>>(sm[1], NUMBER_REGEX,
                            [](auto &sm) { return stoi(sm.str()); }),
        sm[2].str()[0],
        sm[3],
        stoi(sm[4]),
        stoul(sm[5]),
        stoul(sm[6]),
    };
  });

  dump(monkeys);
}
