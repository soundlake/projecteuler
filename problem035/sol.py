def primesUpto(n):
    primes = [2, 3, 5, 7]

    if n <= primes[-1]: return primes

    def isPrime(n):
        maximum = n**.5
        for p in primes:
            if p > maximum: return True
            if n % p == 0: return False

    for candidate in range(primes[-1] + 2, n + 1, 2):
        if isPrime(candidate):
            primes.append(candidate)

    return primes



def findCircularPrimesBelow(limit):
    primes = primesUpto(limit)
    print len(primes), 'number of primes are generated.'

    circulars = [2]

    def isCircular(p):
        s = str(p)
        for c in '02468':
            if c in s: return False
        for i in range(len(s)):
            if int(s) in circulars: return True
            if int(s) not in primes: return False
            s = s[1:] + s[0]
        return True

    for p in primes[1:]:
        if isCircular(p):
            circulars.append(p)
    print len(circulars)

findCircularPrimesBelow(1000 * 1000)
