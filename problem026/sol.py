def findCycle(s):
    l = len(s) / 2
    while l:
        sub = s[-l:]
        pool = s[:-l]
        if sub in pool:
            idx = pool.index(sub)
            found = pool[idx:]
            if found == sub:
                return found
        l -= 1
    return False



def recurringCycle(d):
    s = ''
    n = 1

    while n / d == 0:
        n *= 10

    while True:
        s += str(n / d)
        n = (n % d) * 10
        cycle = findCycle(s)
        if cycle:
            return (d, cycle, len(cycle))



candidates = filter(lambda x: x % 2 and x % 3 and x % 5 and x % 11, range(2, 1000))
candidates = map(recurringCycle, candidates)
print max(candidates, key = lambda x: x[2])
