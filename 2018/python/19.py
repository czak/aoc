import time

def addr(a, b, c, R): R[c] = R[a] + R[b]
def addi(a, b, c, R): R[c] = R[a] + b
def mulr(a, b, c, R): R[c] = R[a] * R[b]
def muli(a, b, c, R): R[c] = R[a] * b
def banr(a, b, c, R): R[c] = R[a] & R[b]
def bani(a, b, c, R): R[c] = R[a] & b
def borr(a, b, c, R): R[c] = R[a] | R[b]
def bori(a, b, c, R): R[c] = R[a] | b
def setr(a, b, c, R): R[c] = R[a]
def seti(a, b, c, R): R[c] = a
def gtir(a, b, c, R): R[c] = 1 if a > R[b] else 0
def gtri(a, b, c, R): R[c] = 1 if R[a] > b else 0
def gtrr(a, b, c, R): R[c] = 1 if R[a] > R[b] else 0
def eqir(a, b, c, R): R[c] = 1 if a == R[b] else 0
def eqri(a, b, c, R): R[c] = 1 if R[a] == b else 0
def eqrr(a, b, c, R): R[c] = 1 if R[a] == R[b] else 0

def compute(instruction, R, IP, IP_BINDING):
    R = R.copy()
    R[IP_BINDING] = IP
    op = instruction[0]
    op(*instruction[1:], R)
    IP = R[IP_BINDING]
    IP += 1
    return (R, IP)

# (mulr, 1, 3, 4),   # 3        # 6
# (eqrr, 4, 5, 4),   # 4        # 8
# (addr, 4, 2, 2),   # 5        # 9
# (addi, 2, 1, 2),   # 6        # 10
# (addi, 3, 1, 3),   # 8
# (gtrr, 3, 5, 4),   # 9
# (addr, 2, 4, 2),   # 10
# (seti, 2, 5, 2),   # 11

PROGRAM = [
    (seti, 123, 0, 2),
    (bani, 2, 456, 2),
    (eqri, 2, 72, 2),
    (addr, 2, 5, 5),
    (seti, 0, 0, 5),
    (seti, 0, 5, 2),
    (bori, 2, 65536, 4),
    (seti, 6718165, 9, 2),
    (bani, 4, 255, 3),
    (addr, 2, 3, 2),
    (bani, 2, 16777215, 2),
    (muli, 2, 65899, 2),
    (bani, 2, 16777215, 2),
    (gtir, 256, 4, 3),
    (addr, 3, 5, 5),
    (addi, 5, 1, 5),
    (seti, 27, 8, 5),
    (seti, 0, 4, 3),
    (addi, 3, 1, 1),
    (muli, 1, 256, 1),
    (gtrr, 1, 4, 1),
    (addr, 1, 5, 5),
    (addi, 5, 1, 5),
    (seti, 25, 8, 5),
    (addi, 3, 1, 3),
    (seti, 17, 3, 5),
    (setr, 3, 6, 4),
    (seti, 7, 9, 5),
    (eqrr, 2, 0, 3),
    (addr, 3, 5, 5),
    (seti, 5, 1, 5),
]

IP_BINDING = 5

R = [0, 0, 0, 0, 0, 0]
IP = 0

while IP >= 0 and IP < len(PROGRAM):
    (R, IP) = compute(PROGRAM[IP], R, IP, IP_BINDING)
    print(R, IP)
    time.sleep(0.1)
