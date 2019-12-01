import re

def good_pair(l1, l2):
    return (l1 in range(65, 91) and l2 == l1 + 32) or (l1 in range(97, 123) and l2 == l1 - 32)

def find_reactions(a):
    reactions = []
    for i in range(len(a)-1):
        if len(reactions) > 0 and reactions[-1] == i - 1:
            continue
        if good_pair(a[i], a[i+1]):
            reactions.append(i)
    return reactions

# ---

with open('input5.txt') as f:
    a = f.read().strip()

# a = 'dabAcCaCBAcCcaDA'

mapping = {}
for c in map(chr, range(ord('a'), ord('z')+1)):
    mapping[c.lower()] = c.upper()
    mapping[c.upper()] = c.lower()

def reduce(polymer):
    stack = []
    for c in polymer:
        if stack and c == mapping[stack[-1]]:
            stack.pop()
        else:
            stack.append(c)
    return stack

lenghts = []
for c in range(65, 91):
    pat = f"{chr(c)}|{chr(c+32)}"
    polymer = reduce(re.sub(pat, '', a))
    lenghts.append(len(polymer))

print(min(lenghts))
