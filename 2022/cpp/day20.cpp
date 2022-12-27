#include "stdafx.h"

istringstream example{R"(1
2
-3
3
-2
0
4)"};

using ll = long long;

struct Node
{
  ll n;
  Node* prev;
  Node* next;
};

void dump(Node* head)
{
  Node* node = head;

  do {
    cout << node->n << " -> ";
    node = node->next;
  } while (node != head);

  cout << endl;
}

vector<Node*> make_list(vector<ll> numbers)
{
  assert(numbers.size() > 0);

  Node* head{new Node{numbers[0]}};
  Node* prev = head;

  vector<Node*> nodes{head};

  for (size_t i = 1; i < numbers.size(); i++) {
    Node* node = new Node{numbers[i], prev};
    prev->next = node;
    prev = node;

    nodes.push_back(node);
  }

  // tie ends
  prev->next = head;
  head->prev = prev;

  return nodes;
}

vector<ll> parse(istream& input)
{
  vector<ll> numbers{};
  ll n;
  while (input >> n)
    numbers.push_back(n);
  return numbers;
}

void mix(vector<Node*>& nodes)
{
  for (auto node : nodes) {
    ll shift = abs(node->n) % (nodes.size() - 1);
    if (shift == 0) continue;

    // remove from list
    Node* prev = node->prev;
    Node* next = node->next;
    node->next->prev = prev;
    node->prev->next = next;

    if (node->n < 0) {
      prev = node->prev;
      for (int i = 0; i < shift; i++) {
        prev = prev->prev;
      }
    } else if (node->n > 0) {
      prev = node;
      for (int i = 0; i < shift; i++) {
        prev = prev->next;
      }
    }
    next = prev->next;

    // insert between prev-next
    prev->next = node;
    next->prev = node;
    node->prev = prev;
    node->next = next;
  }
}

ll solve(const vector<ll>& numbers, int num_mixes = 1)
{
  auto nodes = make_list(numbers);

  for (int m = 0; m < num_mixes; m++)
    mix(nodes);

  // find 0
  Node* node = nodes[0];
  while (node->n != 0)
    node = node->next;

  // sum at 1000, 2000, 3000
  ll sum = 0;
  for (int j = 0; j < 3; j++) {
    for (int i = 0; i < 1000; i++)
      node = node->next;
    sum += node->n;
  }

  return sum;
}

int main()
{
  auto numbers = parse(cin);

  cout << "Part 1: " << flush << solve(numbers) << '\n';

  for (auto& n : numbers)
    n *= 811589153;

  cout << "Part 2: " << flush << solve(numbers, 10) << '\n';
}
