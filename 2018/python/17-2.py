import re

lines = open('17-2.out').readlines()

count = 0
for l in lines:
    for piece in l.split('#'):
        print(piece)
        if set(piece) == set(['~']):
            count += len(piece)
print(count)
