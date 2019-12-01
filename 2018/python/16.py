from parse import parse
from pprint import pprint

R = [0, 0, 0, 0]

def addr(a, b, c): R[c] = R[a] + R[b]
def addi(a, b, c): R[c] = R[a] + b
def mulr(a, b, c): R[c] = R[a] * R[b]
def muli(a, b, c): R[c] = R[a] * b
def banr(a, b, c): R[c] = R[a] & R[b]
def bani(a, b, c): R[c] = R[a] & b
def borr(a, b, c): R[c] = R[a] | R[b]
def bori(a, b, c): R[c] = R[a] | b
def setr(a, b, c): R[c] = R[a]
def seti(a, b, c): R[c] = a
def gtir(a, b, c): R[c] = int(a > R[b])
def gtri(a, b, c): R[c] = int(R[a] > b)
def gtrr(a, b, c): R[c] = int(R[a] > R[b])
def eqir(a, b, c): R[c] = int(a == R[b])
def eqri(a, b, c): R[c] = int(R[a] == b)
def eqrr(a, b, c): R[c] = int(R[a] == R[b])

OPS = {
        'addr': addr, 
        'addi': addi, 
        'mulr': mulr, 
        'muli': muli, 
        'banr': banr, 
        'bani': bani, 
        'borr': borr, 
        'bori': bori, 
        'setr': setr, 
        'seti': seti, 
        'gtir': gtir, 
        'gtri': gtri, 
        'gtrr': gtrr, 
        'eqir': eqir, 
        'eqri': eqri, 
        'eqrr': eqrr,
        }

OPCODES = {
        0: addr,
        1: eqri,
        2: eqir,
        3: eqrr,
        4: gtir,
        5: addi,
        6: banr,
        7: gtri,
        8: bori,
        9: muli,
        10: seti,
        11: gtrr,
        12: setr,
        13: borr,
        14: mulr,
        15: bani,
        }

def load_samples():
    blocks = open('16-1.in').read().split('\n\n')
    samples = []
    for b in blocks:
        lines = b.splitlines()
        sample = {}
        sample['before'] = list(parse('Before: [{:d}, {:d}, {:d}, {:d}]', lines[0]).fixed)
        sample['instruction'] = parse('{:d} {:d} {:d} {:d}', lines[1]).fixed
        sample['after'] = list(parse('After:  [{:d}, {:d}, {:d}, {:d}]', lines[2]).fixed)
        samples.append(sample)
    return samples

def load_code():
    instructions = []
    for line in open('16-2.in'):
        i = parse('{:d} {:d} {:d} {:d}', line).fixed
        instructions.append(i)
    return instructions

def simulate(sample):
    global R
    ops = set()
    for name, op in OPS.items():
        R = sample['before'].copy()
        op(*sample['instruction'][1:])
        if R == sample['after']:
            ops.add(name)
    return ops

R = [0, 0, 0, 0]

for instruction in load_code():
    op = OPCODES[instruction[0]]
    op(*instruction[1:])

print(R)
