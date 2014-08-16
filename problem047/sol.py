primes = [2, 3]
def extendPrimesUpto(limit):
    if limit < primes[-1]: return primes

    def isPrime(n):
        if n in primes: return True
        for p in primes:
            if p > n**.5: return True
            if n % p == 0: return False
        return True

    o1 = (5 - primes[-1]) % 6
    o2 = (9 - primes[-1]) % 6
    if o1 > o2: o1, o2 = o2, o1
    for c in range(primes[-1]+2, limit+1, 6):
        if c+o1<=limit and isPrime(c+o1): primes.append(c+o1)
        if c+o2<=limit and isPrime(c+o2): primes.append(c+o2)

    return primes



def factorize(n):
    pfs = {}
    if n > primes[-1]: extendPrimesUpto(n*2)

    while n > 1:
        for p in primes:
            if n % p: continue
            n /= p
            if p in pfs: pfs[p] += 1
            else: pfs[p] = 1
            break

    return pfs



def isConsecutive(t):
    return all(t[i]+1 == t[i+1] for i in range(len(t)-1))



def findConsecutiveNumbersWithSameNumberOfDistinctPrimeFactor(N):
    n = 1
    a = []
    while 1:
        if len(factorize(n)) == N: a.append(n)
        if len(a) > N: a = a[-N:]
        if len(a) == N and isConsecutive(a): return a
        n += 1



print findConsecutiveNumbersWithSameNumberOfDistinctPrimeFactor(4)
