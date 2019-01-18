#include <iostream>

constexpr int STEP = 367;

struct Node {
  int val;
  Node* next;
};

void print_list(const Node* head) {
  const Node* node = head;
  do {
    std::cout << node->val << "  ";
    node = node->next;
  } while (node != head);
  std::cout << '\n';
}

int main() {
  Node* head = new Node { 0, nullptr };
  head->next = head;

  Node* current = head;

  for (int i=1; i<=2017; i++) {
    // skip STEP steps
    for (int j=0; j<STEP; j++) {
      current = current->next;
    }

    // insert new value
    Node* node = new Node { i, current->next };
    current->next = node;

    // update current
    current = current->next;
  }

  std::cout << "Part 1: " << current->next->val << '\n';

  for (int i=2018; i<=50000000; i++) {
    // skip STEP steps
    for (int j=0; j<STEP; j++) {
      current = current->next;
    }

    // insert new value
    Node* node = new Node { i, current->next };
    current->next = node;

    // update current
    current = current->next;

    if (i % 100000 == 0) {
      std::cout << i << '\n';
    }
  }

  std::cout << "Part 2: " << head->next->val << '\n';
}
