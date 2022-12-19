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

struct Operation
{
  char operation;
  string operand;

  uint64_t operator()(uint64_t n)
  {
    if (operand == "old") {
      if (operation == '*') return n * n;
      else if (operation == '+') return n + n;
      else throw new runtime_error{"unsupported operation"};
    } else {
      if (operation == '*') return n * stoul(operand);
      else if (operation == '+') return n + stoi(operand);
      else throw new runtime_error{"unsupported operation"};
    }
  }
};

struct Test
{
  int div;
  size_t t;
  size_t f;

  size_t operator()(int n) { return n % div == 0 ? t : f; }
};

struct Monkey
{
  list<int> items;
  Operation op;
  Test test;
  uint64_t count{0};
};

ostream& operator<<(ostream& out, const Monkey& m)
{
  out << "{";
  // clang-format off
  dbg::pretty_print(out, m.items); out << ", ";
  dbg::pretty_print(out, m.op.operation); out << ", ";
  dbg::pretty_print(out, m.op.operand); out << ", ";
  dbg::pretty_print(out, m.test.div); out << ", ";
  dbg::pretty_print(out, m.test.t); out << ", ";
  dbg::pretty_print(out, m.test.t); out << ", ";
  dbg::pretty_print(out, m.count);
  // clang-format on
  out << "}";
  return out;
}

uint64_t solve(vector<Monkey> monkeys, int num_rounds, function<int(uint64_t)> post)
{
  for (int i = 0; i < num_rounds; i++) {
    for (auto& [items, operation, test, count] : monkeys) {
      while (!items.empty()) {
        int item = post(operation(items.front()));
        monkeys[test(item)].items.push_back(item);
        items.pop_front();
        count++;
      }
    }
  }

  sort(monkeys.begin(), monkeys.end(), [](const auto& m1, const auto& m2) { return m1.count > m2.count; });

  return (uint64_t)monkeys[0].count * (uint64_t)monkeys[1].count;
}

int main()
{
  string data{istreambuf_iterator{cin}, {}};
  int mod = 1;
  auto monkeys = parse_to<vector<Monkey>>(data, MONKEY_REGEX, [&mod](auto& sm) {
    mod *= stoi(sm[4]);
    return Monkey{
      parse_to<list<int>>(sm[1], NUMBER_REGEX, [](auto& sm) { return stoi(sm.str()); }),
      {sm[2].str()[0], sm[3]},
      {stoi(sm[4]), stoul(sm[5]), stoul(sm[6])},
    };
  });

  cout << "Part 1: " << solve(monkeys, 20, [](uint64_t i) { return i / 3; }) << '\n';
  cout << "Part 2: " << solve(monkeys, 10000, [mod](uint64_t i) { return i % mod; }) << '\n';
}
