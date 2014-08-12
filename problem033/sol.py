from fractions import Fraction as F
from fractions import gcd



fractions = []
def check(aa, bb, a, b):
    if F(aa, bb) == F(a, b):
        print aa, '/', bb, '=', a, '/', b
        fractions.append((a, b))
for a in range(1, 8):
    for b in range(a+1, 9):
        for c in range(b+1, 10):
            ab = a*10 + b
            ac = a*10 + c
            ba = b*10 + a
            bc = b*10 + c
            ca = c*10 + a
            cb = c*10 + b
            # case 1 : ab/ca
            check(ab, ca, b, c)
            # case 2 : ab/bc
            check(ab, bc, a, c)
            # case 3 : ba/cb
            check(ba, cb, a, c)
            # case 4 : ac/cb
            check(ac, cb, a, b)
product = reduce(lambda x, y: (x[0] * y[0], x[1] * y[1]), fractions)
answer = product[1] / gcd(product[0], product[1])
print answer
