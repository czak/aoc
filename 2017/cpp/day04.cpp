#include <iostream>
#include <sstream>
#include <iterator>
#include <vector>
#include <unordered_set>
#include <algorithm>

using namespace std;

vector<string> split(string s) {
  istringstream is {s};
  return {
    istream_iterator<string>{is},
    istream_iterator<string>{},
  };
}

template<class Hash=hash<string>, class Equal=equal_to<string>>
struct is_valid {
  bool operator()(const vector<string>& v) const {
    unordered_set<string, Hash, Equal> s(v.begin(), v.end());
    return v.size() == s.size();
  }
};

struct ahash {
  size_t operator()(const string& r) const {
    string s = r;
    sort(s.begin(), s.end());
    return hash<string>()(s);
  }
};

struct aequal {
  bool operator()(const string& lhs, const string& rhs) const {
    string slhs = lhs;
    sort(slhs.begin(), slhs.end());
    string srhs = rhs;
    sort(srhs.begin(), srhs.end());
    return slhs == srhs;
  }
};

int main() {
  vector<vector<string>> lines;
  string line;
  while (getline(cin, line)) {
    lines.push_back(split(line));
  }

  int count = count_if(lines.begin(), lines.end(), is_valid<>());
  cout << "Part 1: " << count << "\n";

  count = count_if(lines.begin(), lines.end(), is_valid<ahash, aequal>());
  cout << "Part 2: " << count << "\n";
}
