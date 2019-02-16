#include <iostream>
#include <sstream>
#include <vector>
#include <tuple>
#include <map>
using namespace std;

using instruction = tuple<string, string, string>;

int main() {
  vector<instruction> program;

  string s;
  string op, arg1, arg2;
  while (getline(cin, s)) {
    istringstream in(s);
    in >> op >> arg1 >> arg2;
    program.push_back({op, arg1, arg2});
  }

  // map<string, int> registers {
  //   {"a", 1},
  //   {"b", 0},
  //   {"c", 0},
  //   {"d", 0},
  //   {"e", 0},
  //   {"f", 0},
  //   {"g", 0},
  //   {"h", 0},
  // };
  //
  // int ip = 0;
  //


  // map<string, int> registers {
  //   {"a", 1},
  //   {"b", 107900},
  //   {"c", 124900},
  //   {"d", 107890},
  //   {"e", 107900},
  //   {"f", 0},
  //   {"g", -10},
  //   {"h", 0},
  // };
  //
  // int ip = 23;

  // map<string, int> registers {
  //   {"a", 1},
  //   {"b", 107917},
  //   {"c", 124900},
  //   {"d", 107910},
  //   {"e", 107917},
  //   {"f", 1},
  //   {"g", -7},
  //   {"h", 1},
  // };
  //
  // int ip = 23;

  map<string, int> registers {
    {"a", 1},
    {"b", 107934},
    {"c", 124900},
    {"d", 107900},
    {"e", 107934},
    {"f", 0},
    {"g", -34},
    {"h", 1},
  };

  int ip = 23;

  auto getval = [&registers](string arg) {
    if (isalpha(arg[0])) {
      return registers[arg];
    } else {
      return stoi(arg);
    }
  };

  while (ip >= 0 && ip < (long) program.size()) {
    std::tie(op, arg1, arg2) = program[ip];

    if (ip == 8 || ip == 15) {
      cout << "a: " << registers["a"] << " ";
      cout << "b: " << registers["b"] << " ";
      cout << "c: " << registers["c"] << " ";
      cout << "d: " << registers["d"] << " ";
      cout << "e: " << registers["e"] << " ";
      cout << "f: " << registers["f"] << " ";
      cout << "g: " << registers["g"] << " ";
      cout << "h: " << registers["h"] << "   | ";
      cout << "ip: " << ip << " | " << op << " " << arg1 << " " << arg2 << '\n';
    }

    if (op == "set") {
      registers[arg1] = getval(arg2);
    } else if (op == "sub") {
      registers[arg1] -= getval(arg2);
    } else if (op == "mul") {
      registers[arg1] *= getval(arg2);
    } else if (op == "jnz") {
      if (getval(arg1) != 0) {
        ip += getval(arg2);
        continue;
      }
    }

    ip++;
  }
}
