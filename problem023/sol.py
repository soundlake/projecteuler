MAX = 28123



# find abundant list
def pf(n):
    f = {1}
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            f.add(i)
            f.add(n / i)
    return list(f)

def isAbundant(n):
    return sum(pf(n)) > n

abundant = filter(isAbundant, range(1, MAX + 1))



# find sums of two abundant numbers
sums = set([24])
for a in range(len(abundant)):
    for b in range(a, len(abundant)):
        sums.add(abundant[a] + abundant[b])

result = filter(lambda x: not x in sums, range(1, MAX + 1))

print sum(result)
