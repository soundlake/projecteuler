import json



# read data from the file
datafile = '../data/prime.json'
try:
    f = open(datafile, 'r')
    primes = json.load(f)
    f.close()
except:
    primes = [2, 3]



def isPrime(n):
    if n < primes[0]:
        return False
    if n in primes:
        return True
    for prime in primes:
        if prime > n**.5:
            return True
        if n % prime == 0:
            return False
    for candidate in range(primes[-1] + 2, int(n**.5), 2):
        if isPrime(candidate):
            print 'bang'
            primes.append(candidate)
            if n % candidate == 0:
                return False
    return True



# initialize the placeholder
max_a = 0
max_b = 0
max_v = 0

# extend prime table for using b
for n in range(primes[-1], 1000, 2):
    if isPrime(n):
        primes.append(n)
bs = primes[::]

def prints(a, b, n):
    def printA(a):
        if a > 0: return ' + ' + str(a) + 'n'
        elif a == 0: return ''
        else: return ' - ' + str(a * -1) + 'n'
    def printB(b):
        if b >= 0: return ' + ' + str(b)
        else: return ' - ' + str(b)
    print '' + str(n) + '^2' + printA(a) + printB(b)

for a in range(-999, 1000):
    for b in bs:
        for i in (-1, 1):
            length = 1
            #prints(a, b, length)
            while isPrime(length**2 + a*length + b):
                length += 1
            if length > max_v:
                max_a, max_b, max_v = a, b, length
print 'answer:', (max_a * max_b)



# write data to the file
f = open(datafile, 'w')
json.dump(sorted(primes), f)
f.close()
