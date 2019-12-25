def modpow(x, e, m):
    y = 1
    while e > 0:
        if e % 2 == 0:
            x = (x * x) % m
            e = e / 2
        else:
            y = (x * y) % m
            e = e - 1
    return y

card = 2019
size = 10007

SEQUENCE = [
(modpow(66, size-2, size), 0),
(1, -2068),
(modpow(8, size-2, size), 0),
(1, -6565),
(modpow(22, size-2, size), 0),
(1, -8629),
(modpow(56, size-2, size), 0),
(1, -697),
(modpow(58, size-2, size), 0),
(1, 4957),
(modpow(71, size-2, size), 0),
(1, -4506),
(modpow(39, size-2, size), 0),
(1, 6144),
(modpow(48, size-2, size), 0),
(1, 1392),
(modpow(51, size-2, size), 0),
(1, -8043),
(modpow(30, size-2, size), 0),
(1, 7798),
(modpow(25, size-2, size), 0),
(-1, size-1),
(modpow(43, size-2, size), 0),
(1, 1048),
(modpow(63, size-2, size), 0),
(1, 257),
(-1, size-1),
(modpow(15, size-2, size), 0),
(-1, size-1),
(modpow(12, size-2, size), 0),
(-1, size-1),
(1, 3316),
(modpow(68, size-2, size), 0),
(1, -4495),
(modpow(4, size-2, size), 0),
(1, -421),
(modpow(11, size-2, size), 0),
(1, 7629),
(modpow(32, size-2, size), 0),
(1, -3956),
(modpow(33, size-2, size), 0),
(1, -596),
(modpow(42, size-2, size), 0),
(1, 8505),
(-1, size-1),
(1, 4215),
(modpow(74, size-2, size), 0),
(1, 9999),
(modpow(7, size-2, size), 0),
(-1, size-1),
(modpow(71, size-2, size), 0),
(1, 6836),
(modpow(27, size-2, size), 0),
(1, 188),
(modpow(45, size-2, size), 0),
(-1, size-1),
(modpow(17, size-2, size), 0),
(1, -6659),
(-1, size-1),
(1, -8919),
(modpow(23, size-2, size), 0),
(1, 7758),
(modpow(58, size-2, size), 0),
(1, -9377),
(modpow(51, size-2, size), 0),
(1, -8010),
(-1, size-1),
(1, -8058),
(modpow(57, size-2, size), 0),
(-1, size-1),
(modpow(7, size-2, size), 0),
(1, -1977),
(-1, size-1),
(1, -4748),
(modpow(55, size-2, size), 0),
(1, -2901),
(-1, size-1),
(1, 4362),
(modpow(65, size-2, size), 0),
(1, -4367),
(modpow(51, size-2, size), 0),
(1, 2133),
(-1, size-1),
(modpow(15, size-2, size), 0),
(-1, size-1),
(modpow(28, size-2, size), 0),
(1, -5331),
(modpow(41, size-2, size), 0),
(1, -5157),
(modpow(68, size-2, size), 0),
(1, 4776),
(-1, size-1),
(modpow(28, size-2, size), 0),
(1, 2005),
(modpow(14, size-2, size), 0),
(-1, size-1),
(1, 1341),
(-1, size-1),
(1, 7623),
(modpow(36, size-2, size), 0),
]

a = 1
b = 0

for na, nb in reversed(SEQUENCE):
    a = (a * na) % size
    b = (b * na + nb) % size

print(a, b)

print((a * 2519 + b) % size)
