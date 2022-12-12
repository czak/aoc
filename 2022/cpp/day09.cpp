#include "include/stdafx.h"

istringstream example { R"(R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2)" };

// Y positive pointing down
struct vec2 {
    int x;
    int y;

    void move(char dir) {
        switch (dir) {
            case 'U': y -= 1; break;
            case 'L': x -= 1; break;
            case 'D': y += 1; break;
            case 'R': x += 1; break;
        }
    }

    void follow(const vec2& other) {
        int dx = other.x - x;
        int dy = other.y - y;
        if (abs(dx) <= 1 && abs(dy) <= 1) return;
        if (dx != 0) dx /= abs(dx);
        if (dy != 0) dy /= abs(dy);
        x += dx;
        y += dy;
    }
};

template<>
struct std::hash<vec2> {
  size_t operator()(const vec2& v) const {
    return hash<int>{}(v.x) ^ (hash<int>{}(v.y) << 1);
  }
};

template<>
struct std::equal_to<vec2> {
    bool operator()(const vec2& l, const vec2& r) const {
        return l.x == r.x && l.y == r.y;
    }
};

ostream& operator<<(ostream& out, const vec2& v) {
    out << '(' << v.x << ',' << v.y << ')';
    return out;
}

using instruction = pair<char, int>;

vector<instruction> parse(istream& input) {
    vector<instruction> v;

    char dir;
    int dist;
    while (input >> dir >> dist) {
        v.push_back({ dir, dist });
    }

    return v;
}

int rope(const vector<instruction>& instructions, size_t length) {
    unordered_set<vec2> visited{};
    vector<vec2> rope{ length };

    for (auto& [dir, dist] : instructions) {
        for (int i = 0; i < dist; i++) {
            rope[0].move(dir);
            for (int j = 1; j < length; j++)
                rope[j].follow(rope[j-1]);
            visited.insert(rope[length-1]);
        }
    }

    return visited.size();
}

int main() {
    auto input = parse(cin);

    cout << "Part 1: " << rope(input, 2) << '\n';
    cout << "Part 2: " << rope(input, 10) << '\n';
}
