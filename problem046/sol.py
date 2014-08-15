def twiceSqaure(n):
    return (n/2)**.5



def isTwiceASqaure(n):
    t = twiceSqaure(n)
    return t == int(t)



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



def findSmallestNonGoldbach():
    limit = 1024
    oddComposite = 9
    while True:
        if oddComposite > primes[-2]:
            limit *= 2
            extendPrimesUpto(limit)
        if oddComposite not in primes:
            isGoldbach = False
            for p in primes:
                if p > oddComposite: break
                if isTwiceASqaure(oddComposite - p):
                    isGoldbach = True
                    break
            if not isGoldbach:
                return oddComposite
        oddComposite += 2



print findSmallestNonGoldbach()
