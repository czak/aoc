#include "include/stdafx.h"

istringstream example { R"(30373
25512
65332
33549
35390)" };

using grid = vector<vector<int>>;

auto parse(istream& in) {
    grid trees;
    for (string line; getline(in, line);) {
        vector<int> row;
        for (char ch : line) row.push_back(ch - '0');
        trees.push_back(row);
    }
    return trees;
}

bool visible_in(grid& trees, int height, int x, int y, int dx, int dy) {
    if (x == 0 || y == 0 || x == trees[0].size() - 1 || y == trees.size() - 1)
        return true;
    if (trees[y+dy][x+dx] >= height)
        return false;
    return visible_in(trees, height, x+dx, y+dy, dx, dy);
};

bool visible(grid& trees, int x, int y) {
    int height = trees[y][x];
    return
        visible_in(trees, height, x, y, -1, 0) ||
        visible_in(trees, height, x, y, 1, 0) ||
        visible_in(trees, height, x, y, 0, -1) ||
        visible_in(trees, height, x, y, 0, 1);
}

int measure_in(grid& trees, int height, int x, int y, int dx, int dy, int count) {
    if (x == 0 || y == 0 || x == trees[0].size() - 1 || y == trees.size() - 1)
        return count;
    if (trees[y+dy][x+dx] >= height)
        return count+1;
    return measure_in(trees, height, x+dx, y+dy, dx, dy, count+1);
}

int measure(grid& trees, int x, int y) {
    int height = trees[y][x];
    return
        measure_in(trees, height, x, y, -1, 0, 0) *
        measure_in(trees, height, x, y, 1, 0, 0) *
        measure_in(trees, height, x, y, 0, -1, 0) *
        measure_in(trees, height, x, y, 0, 1, 0);
}

int main() {
    auto trees = parse(cin);

    int sum = 0, max = 0;
    for (int y = 0; y < trees.size(); y++) {
        for (int x = 0; x < trees[y].size(); x++) {
            // Part 1
            sum += visible(trees, x, y);

            // Part 2
            int score = measure(trees, x, y);
            max = score > max ? score : max;
        }
    }

    dbg(sum);
    dbg(max);
}
