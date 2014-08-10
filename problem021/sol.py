# functions
def d(n):
    d = [1]
    for i in range(2, int(n ** 0.5) + 1):
        if n % i:
            continue
        d.append(i)
        if i != n / i:
            d.append(n / i)
    return sum(d)

# initialize
ds = set([])
for n in range(2, 10000):
    m = d(n)
    if n != m and n == d(m):
        ds.add(n)
print sum(list(ds))
