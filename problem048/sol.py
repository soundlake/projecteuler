def f(limit):
    s = 0
    for n in range(1, limit+1):
        p = 1
        for i in range(n):
            p = (p * n) % (10 ** 10)
        s = (s + p) % (10 ** 10)
    return s

print f(1000)
