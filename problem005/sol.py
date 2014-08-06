#initialize
primes = [2, 3, 5, 7, 11, 13, 17, 19]
factors = {}
for p in primes:
    factors[str(p)] = 0



# sub function
def factorize(n):
    d = {}
    for p in primes:
        d[str(p)] = 0

    if n in primes:
        d[str(n)] = 1
    else:
        for p in primes:
            while n % p == 0:
                if str(p) in d:
                    d[str(p)] += 1
                n /= p
    return d



# main logic
for i in range(2, 21):
    factorized = factorize(i);
    for k in factorized:
        if (factors[k] < factorized[k]):
            factors[k] = factorized[k]

answer = 1
for k in factors:
    answer *= int(k) ** factors[k]
print answer
