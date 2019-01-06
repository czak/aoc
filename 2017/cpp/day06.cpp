#include <iostream>
#include <algorithm>
#include <iterator>
#include <array>
#include <unordered_set>
#include <sstream>
#include <string>
#include <vector>
#include <utility>

using bank_array = std::vector<int>;

struct bank_hash {
  size_t operator()(const bank_array& banks) const {
    size_t h = 0;
    for (auto& b: banks) {
      h ^= std::hash<int>{}(b);
    }
    return h;
  }
};

std::ostream& operator<<(std::ostream& os, const bank_array& banks) {
  std::copy(banks.begin(), banks.end(), std::ostream_iterator<int>(os, " "));
  return os;
}

std::pair<int, int> solve(bank_array& banks) {
  int cycles = 0;
  std::unordered_set<bank_array, bank_hash> seen {banks};
  std::vector<bank_array> vseen {banks};
  while (true) {
    auto it = std::max_element(banks.begin(), banks.end());
    auto max = *it;
    *it = 0;
    while (max--) {
      if (++it == banks.end()) it = banks.begin();
      (*it)++;
    }
    cycles++;
    std::cout << "After cycle " << cycles << ": " << banks << "\n";
    if (seen.count(banks)) {
      auto first = find(vseen.begin(), vseen.end(), banks);
      return {cycles, std::distance(first, vseen.end())};
    }
    seen.insert(banks);
    vseen.push_back(banks);
  }
}

int main() {
  bank_array banks {};
  int b;
  while (std::cin >> b) {
    banks.push_back(b);
  }
  auto res = solve(banks);
  std::cout << res.first << ", " << res.second << "\n";
}
