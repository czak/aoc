# s = [3, 7]
s = bytearray([3, 7])

c1 = 0
c2 = 1

pattern = bytearray([0, 7, 4, 5, 0, 1])

while True:
    n = s[c1] + s[c2]
    if n >= 10:
        s.append(1)
        if s[-6:] == pattern:
            break
    s.append(n % 10)
    if s[-6:] == pattern:
        break

    c1 = (c1 + s[c1] + 1) % len(s)
    c2 = (c2 + s[c2] + 1) % len(s)

print(len(s) - len(pattern))
