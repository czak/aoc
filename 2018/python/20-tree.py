from pprint import pprint
from copy import deepcopy
from re import findall

s = '(ENWWW(NEEE|SSE(EE|N))D)'

class Node:
    def __init__(self, token=None):
        self.token = token
        self.parent = None
        self.children = []
    def add_child(self, token=None):
        node = Node(token)
        node.parent = self
        self.children.append(node)
        return node

def traverse(node):
    if node.token == '*':
        if not node.children: return []
        current = traverse(node.children[0])
        for c in node.children[1:]:
            current = [l + r for l in current for r in traverse(c)]
        return current
    elif node.token == '|':
        if not node.children: return []
        current = traverse(node.children[0])
        for c in node.children[1:]:
            current.extend(traverse(c))
        return current
    else:
        return [node.token]

def build_tree(tokens):
    root = Node('*')
    node = root.add_child()
    for t in tokens:
        if t == '(':
            if node.token == 
        else:
    return root

tokens = findall(r'(\w+|\W)', s)
print(tokens)

root = build_tree(tokens)
# l = traverse(root)
# print(l)
