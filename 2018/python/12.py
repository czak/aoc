from parse import parse

lines = [line.strip() for line in open('12.in')]

state = parse('initial state: {}', lines[0])[0]
state = set(i for i, pot in enumerate(state) if pot == '#')

matches = set()
for line in lines[2:]:
    pattern, result = parse('{} => {}', line)
    if result == '#':
        matches.add(frozenset(i-2 for i, p in enumerate(pattern) if p == '#'))

def grow(i):
    area = state & {i-2, i-1, i, i+1, i+2}
    return set(n-i for n in area) in matches

def format_state(state, min=-2):
    s = []
    for i in range(min, max(state) + 10):
        if i in state:
            s.append('#')
        else:
            s.append('.')
    return ''.join(s)

for gen in range(1, 200):
    sets = []
    for p in state:
        sets.append(set(i for i in range(p-5, p+6) if grow(i)))
    state = set.union(*sets)
    print(gen, sum(state), len(state), min(state), max(state))
