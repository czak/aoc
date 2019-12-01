numbers = iter(map(int, open('input8.txt').read().split()))

# numbers = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]

def read_tree(it):
    nc, nm = next(it), next(it)
    children = []
    metadata = []
    for _ in range(nc):
        children.append(read_tree(it))
    for _ in range(nm):
        metadata.append(next(it))
    return (children, metadata)

def sum_metadata(tree):
    children, metadata = tree
    total = 0
    for c in children:
        total += sum_metadata(c)
    for m in metadata:
        total += m
    return total

def value(tree):
    children, metadata = tree
    if not children:
        return sum(metadata)
    total = 0
    for i in metadata:
        if i <= len(children):
            total += value(children[i-1])
    return total


root = read_tree(iter(numbers))
print(sum_metadata(root))
print(value(root))

