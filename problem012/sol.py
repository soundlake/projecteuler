import json



# read data from the file
datafile = '../data/prime.json'
try:
    f = open(datafile, 'r')
    primes = json.load(f)
    f.close()
except:
    primes = [2, 3, 5, 7, 11, 13, 17, 19]



# sub function
def isPrime(n):
    if n <= primes[-1]:
        return n in primes
    idx = 0
    factor = primes[idx]
    while factor <= n ** 0.5:
        if n % factor == 0:
            return False
        if idx < len(primes):
            idx += 1
            factor = primes[idx]
            continue
        factor += 2
    primes.append(n)
    return True



def factorize(n):
    d = {}

    if n in primes:
        d[n] = 1
    else:
        for p in primes:
            while n % p == 0:
                if str(p) in d:
                    d[p] += 1
                else:
                    d[p] = 1
                n /= p
    return d



def numOfFactors(n):
    factorized = factorize(n)
    num = 1
    for p in factorized:
        num *= factorized[p] + 1
    return num



# main logic
i = 1
tri = 1
nOF = 0
while nOF < 500:
    i += 1
    tri += i
    nOF = numOfFactors(tri)
    if i % 300 == 0:
        print i, tri, nOF
print i, tri, nOF



# write data to the file
f = open(datafile, 'w')
json.dump(sorted(primes), f)
f.close()
