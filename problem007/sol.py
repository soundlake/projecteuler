import json



datafile = '../data/prime.json'
try:
    f = open(datafile, 'r')
    primes = json.load(f)
    f.close()
except:
    primes = [2, 3, 5, 7, 11, 13, 17, 19]



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
        else:
            factor += 2
    primes.append(n)
    return True



# main
while len(primes) < 10001:
    candidate = primes[-1] + 2
    while not isPrime(candidate):
        candidate += 2
print primes[10000]



f = open(datafile, 'w')
json.dump(sorted(primes), f)
f.close()
