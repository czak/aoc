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

PROGRAM = [
    (seti, 123, 0, 2),       # 0
    (bani, 2, 456, 2),       # 1
    (eqri, 2, 72, 2),        # 2
    (addr, 2, 5, 5),         # 3
    (seti, 0, 0, 5),         # 4
    (seti, 0, 5, 2),         # 5
    (bori, 2, 65536, 4),     # 6
    (seti, 6718165, 9, 2),   # 7
    (bani, 4, 255, 3),       # 8
    (addr, 2, 3, 2),         # 9
    (bani, 2, 16777215, 2),  # 10
    (muli, 2, 65899, 2),     # 11
    (bani, 2, 16777215, 2),  # 12
    (gtir, 256, 4, 3),       # 13
    (addr, 3, 5, 5),         # 14
    (addi, 5, 1, 5),         # 15
    (seti, 27, 8, 5),        # 16
    (seti, 0, 4, 3),         # 17
    (addi, 3, 1, 1),         # 18
    (muli, 1, 256, 1),       # 19
    (gtrr, 1, 4, 1),         # 20
    (addr, 1, 5, 5),         # 21
    (addi, 5, 1, 5),         # 22
    (seti, 25, 8, 5),        # 23
    (addi, 3, 1, 3),         # 24
    (seti, 17, 3, 5),        # 25
    (setr, 3, 6, 4),         # 26
    (seti, 7, 9, 5),         # 27
    (eqrr, 2, 0, 3),         # 28
    (addr, 3, 5, 5),         # 29
    (seti, 5, 1, 5),         # 30
]


def printcpu(end='\n'):
    print(f'IP={IP:2} R=[{R[0]:3}, {R[1]:8}, {R[2]:12}, {R[3]:3}, {R[4]:5}, {R[5]:2}]', end=end)

IP_BINDING = 5
IP = 28
R=[  1,        1,        30842,   1,     1, 27]
# R=[  1,        1,      9617088,   1,     1, 27]

# początek iteracji
# IP=20 R=[  1,      256,     11219205,   0, 96378, 19]
# koniec iteracji (96376 // 256)
# IP=20 R=[  1,    96256,     11219205, 375, 96378, 19]

# IP=20 R=[  1,      256,     15832135,   0, 9682624, 19]
# mogę przyspieszyć do
# IP=20 R=[  1,  9682432,     15832135, 37821, 9682624, 19]

seen = set()

lastnum = -1
while IP >= 0 and IP < len(PROGRAM):
    if IP == 20:
        # turbo
        num = R[4]
        div = num // 256
        R[1] = (div + 1) * 256
        R[3] = div
    if IP == 28:
        num = R[2]
        if num in seen:
            print("last uniq:", lastnum)
            break
        seen.add(num)
        lastnum = num
    instr = PROGRAM[IP]
    # printcpu()
    (R, IP) = compute(instr, R, IP, IP_BINDING)
    # time.sleep(0.1)
