#include "include/stdafx.h"

istringstream example { "A Y\nB X\nC Z" };

const map<string, int> SCORES {
    { "A X", 1 + 3 },
    { "B X", 1 + 0 },
    { "C X", 1 + 6 },
    { "A Y", 2 + 6 },
    { "B Y", 2 + 3 },
    { "C Y", 2 + 0 },
    { "A Z", 3 + 0 },
    { "B Z", 3 + 6 },
    { "C Z", 3 + 3 },
};

const map<string, string> CHOICES {
    { "A X", "A Z" },
    { "B X", "B X" },
    { "C X", "C Y" },
    { "A Y", "A X" },
    { "B Y", "B Y" },
    { "C Y", "C Z" },
    { "A Z", "A Y" },
    { "B Z", "B Z" },
    { "C Z", "C X" },
};

int main() {
    vector<string> lines;
    for (string line; getline(cin, line);) {
        lines.push_back(line);
    }

    int p1 = accumulate(lines.begin(), lines.end(), 0, [](int a, string b) { return a + SCORES.at(b); });
    int p2 = accumulate(lines.begin(), lines.end(), 0, [](int a, string b) { return a + SCORES.at(CHOICES.at(b)); });

    cout << "Part 1: " << p1 << '\n';
    cout << "Part 2: " << p2 << '\n';
}
