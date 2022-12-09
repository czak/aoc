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

int main() {
    string line;
    while (getline(example, line)) {
        dbg(SCORES.at(line));
    }
}
