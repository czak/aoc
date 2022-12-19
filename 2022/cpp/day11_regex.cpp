#include "stdafx.h"

extern const std::regex NUMBER_REGEX{"\\d+"};
extern const std::regex MONKEY_REGEX{
  "Monkey \\d:\\n  Starting items: (.*)\\n  Operation: new = old (.) (.+)\\n  Test: divisible by (\\d+)\\n    If true: "
  "throw to monkey (\\d)\\n    If false: throw to monkey (\\d)"};
