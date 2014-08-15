def isPandigital(n):
    src = '123456789'
    return src[:len(str(n))] == reduce(lambda x,y: x+y, sorted(list(str(n))))



def primesUpto(n):
    primes = [2, 3, 5]
    def isPrime(n):
        for p in primes:
            if p > candidate**.5: return True
            if n % p == 0: return False
        return True
    for candidate in range(primes[-1]+2, n+1, 6):
        if isPrime(candidate):
            primes.append(candidate)
        if isPrime(candidate+4):
            primes.append(candidate+4)
    return primes


# 7-digit number will be the answer
primes = primesUpto(10**7)
for i in range(len(primes)-1, 0, -1):
    if isPandigital(primes[i]):
        print primes[i]
        break
