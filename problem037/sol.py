def truncatablePrimes(number):
    def extandPrimesUpto(n):
        if n <= primes[-1]: return primes
        for c in range(primes[-1]+2, n+1, 2):
            for p in primes:
                if p > c**.5:
                    primes.append(c)
                    break
                if c % p == 0:
                    break

    def isTruncatablePrime(p):
        if p < 10: return False
        for c in '0468':
            if c in str(p): return False
        # from left to right
        for i in range(1, len(str(p))):
            if int(str(p)[i:]) not in primes: return False
        # from right to left
        for i in range(1, len(str(p))):
            if int(str(p)[:-i]) not in primes: return False
        return True

    limit = 10
    primes = [2, 3, 5, 7]
    formerLen = 0
    result = []

    while True:
        for p in primes[formerLen:]:
            if isTruncatablePrime(p): result.append(p)
        if len(result) >= number: break
        limit *= 10
        formerLen = len(primes)
        extandPrimesUpto(limit)

    return result

print sum(truncatablePrimes(11))
