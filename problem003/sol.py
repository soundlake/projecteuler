import json



# read data from the file
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
    while factor < n ** 0.5:
        if n % factor == 0:
            return False
        if idx < len(primes):
            idx += 1
            factor = primes[idx]
            continue
        factor += 2
    primes.append(n)
    return True



# main logic
number = 600851475143
factors = []
idx = 0
prime = primes[idx]
while number > 1:
    # check if the candidate for factor is in primes array
    if idx < len(primes):
        prime = primes[idx]
    # if there isn't, find the next prime
    else:
        candidate = prime + 2
        while not isPrime(candidate):
            candidate += 2
        prime = candidate
    # check if the candidate for factor is real factor
    if number % prime == 0:
        factors.append(prime)
        number /= prime
        continue
    idx += 1
print "answer:", max(factors)



# write data to the file
f = open(datafile, 'w')
json.dump(sorted(primes), f)
f.close()
