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

regex VALVE_RE{"Valve ([A-Z][A-Z]) has flow rate=(\\d+); tunnels? leads? to valves? (.+)"};
regex ID_RE{"[A-Z][A-Z]"};

using Graph = vector<pair<int, int>>;

vector<int> valves{};
vector<pair<int, int>> edges{};
vector<vector<int>> distances{};

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
  unordered_map<string, int> ids;
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

int search(size_t cur, int time, int pressure = 0)
{
  static vector<bool> opened(valves.size());

  dbg(cur, time, pressure);

  if (time < 0) return pressure;

  // find unopened, nonzero valves (indexes)
  vector<size_t> candidates{};
  for (size_t i = 0; i < valves.size(); i++) {
    if (!opened[i] && valves[i] > 0) {
      candidates.push_back(i);
    }
  }

  dbg(candidates);

  // nowhere else to go
  if (candidates.empty()) return pressure;

  // open first
  size_t next = candidates[0];
  opened[next] = true;
  int cost = distances[cur][next];
  int time_left = time - cost - 1; // distance + 1min to open

  // what if cost is higher than time left?
  if (time_left < 0) {
    dbg("time out");
    return pressure;
  }

  cout << "--\n";

  int pres = valves[next] * time_left;
  return search(next, time_left, pressure + pres);
}

int main()
{
  parse(example);
  measure();

  search(0, 30);
}
