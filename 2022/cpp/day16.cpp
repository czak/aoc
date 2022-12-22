#include "stdafx.h"
#include "utils.h"

istringstream example{R"(Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
)"};

const size_t MAX_VALVES = 60;

regex VALVE_RE{"Valve ([A-Z][A-Z]) has flow rate=(\\d+); tunnels? leads? to valves? (.+)"};
regex ID_RE{"[A-Z][A-Z]"};

using Path = vector<pair<size_t, int>>;

unordered_map<string, int> ids{};
vector<int> valves{};
vector<pair<int, int>> edges{};
vector<vector<int>> distances{};
vector<Path> paths{};

void parse(istream& in)
{
  using TmpValve = tuple<string, int, vector<string>>;

  string s{istreambuf_iterator{in}, {}};
  auto tmp = parse_to<vector<TmpValve>>(s, VALVE_RE, [](auto& sm) {
    return TmpValve{
      sm[1],
      stoi(sm[2]),
      parse_to<vector<string>>(sm[3], ID_RE, [](auto& sm) { return sm[0]; }),
    };
  });

  // assign integer ids
  for (size_t i = 0; i < tmp.size(); i++) {
    ids.insert({get<0>(tmp[i]), i});
  }

  // map into valves and edges
  for (size_t i = 0; i < tmp.size(); i++) {
    auto tv = tmp[i];
    valves.push_back(get<1>(tv));
    for (const string& neighbor : get<2>(tv)) {
      int n = ids.at(neighbor);
      edges.push_back({i, n});
    }
  }
}

void measure()
{
  int n = valves.size();

  for (int start = 0; start < n; start++) {
    vector<int> distance(n, 1000000);
    distance[start] = 0;

    for (int i = 0; i < n; i++) {
      for (auto [a, b] : edges) {
        distance[b] = min(distance[b], distance[a] + 1);
      }
    }

    distances.push_back(distance);
  }
}

void search(size_t cur, int time)
{
  static Path path{};

  auto is_opened = [](size_t i) {
    for (auto& [valve, timestamp] : path) {
      if (valve == i) return true;
    }
    return false;
  };

  // candidates:
  // - not yet opened
  // - have flow > 0
  // - can be reached & opened within remaining time
  vector<size_t> candidates{};

  for (size_t i = 0; i < valves.size(); i++) {
    if (is_opened(i)) continue;
    if (valves[i] == 0) continue;
    if (distances[cur][i] >= time - 1) continue;
    candidates.push_back(i);
  }

  // nowhere else to go
  if (candidates.empty()) {
    paths.push_back(path);
    return;
  }

  vector<int> results{};

  for (auto& next : candidates) {
    int cost = distances[cur][next];
    int time_left = time - cost - 1;

    path.push_back({next, time_left});
    search(next, time_left);
    path.pop_back();
  }
}

int part1()
{
  vector<int> scores{};
  transform(paths.begin(), paths.end(), back_inserter(scores), [](Path& p) {
    return accumulate(p.begin(), p.end(), 0,
                      [](int acc, pair<size_t, int>& step) { return acc + valves[step.first] * step.second; });
  });
  return *max_element(scores.begin(), scores.end());
}

int part2()
{
  int best = 0;

  for (auto& path1 : paths) {
    for (auto& path2 : paths) {
      auto it1 = path1.begin();
      auto it2 = path2.begin();
      bool opened[MAX_VALVES]{false};
      int score = 0;

      while (it1 != path1.end() || it2 != path2.end()) {
        auto& it = it1 == path1.end() ? it2 : (it2 == path2.end() || it1->second > it2->second ? it1 : it2);

        auto [n, t] = *it;
        if (opened[n]) break;
        opened[n] = true;
        score += valves[n] * t;
        it++;
      }

      best = max(best, score);
    }
  }

  return best;
}

int main()
{
  parse(cin);
  measure();

  search(ids.at("AA"), 30);
  cout << "Part 1: " << part1() << '\n';

  paths.clear();

  search(ids.at("AA"), 26);
  cout << "Part 2: " << part2() << '\n';
}
