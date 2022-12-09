#include "include/stdafx.h"

istringstream example(R"(1000
2000
3000

4000

5000
6000

7000
8000
9000

10000)");

int main() {
    vector<int> elves = {0};

    string line;
    while (getline(cin, line)) {
        if (line.empty())
            elves.push_back(0);
        else
            elves.back() += stoi(line);
    }

    sort(elves.begin(), elves.end(), greater());

    cout << "Part 1: " << elves[0] << '\n';
    cout << "Part 2: " << accumulate(elves.begin(), elves.begin() + 3, 0) << '\n';
}
