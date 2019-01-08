#include <regex>
#include <iostream>
#include <string>
#include <unordered_set>
using namespace std;

struct node {
  string name;
  int weight;
  unordered_set<node*> children;
};

int main() {
  // 0: wkvfo (177) -> muaiipe, nctisdd, tdyqxhp
  // 1: wkvfo
  // 2: 177
  // 3: muaiipe, nctisdd, tdyqxhp
  regex re {R"((\w+) \((\d+)\)(?: -> (.*))?)"};
  regex child_re {R"(\w+)"};

  unordered_set<string> parents;
  unordered_set<string> children;

  string s;
  smatch m;
  while (getline(cin, s)) {
    regex_match(s, m, re);
    parents.insert(m[1].str());
    string c = m[3].str();
    smatch cm;
    while (regex_search(c, cm, child_re)) {
      children.insert(cm.str());
      c = cm.suffix();
    }
  }

  // no unordered_set difference?
  for (auto& c: children) {
    parents.erase(c);
  }

  for (auto& p: parents) {
    cout << p << '\n';
  }
}
