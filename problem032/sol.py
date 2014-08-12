def perm(origin, r):
    if len(origin) < r: raise TypeError('r should be less than len(origin)')
    source = sorted(map(lambda x: str(x), origin))
    if r < 2: return source
    length = len(source)
    arr = []
    for i in range(length):
        el_a = source[i]
        for el_b in perm(source[:i] + source[i+1:], r - 1):
            arr.append(el_a + el_b)
    return sorted(arr)

perms = perm(range(1, 10), 9)
products = set([])
for p in perms:
    for alen in range(1, 5):
        a = int(p[:alen])
        for blen in range(alen, 5):
            b = int(p[alen:alen+blen])
            c = a*b
            if c == int(p[alen+blen:]):
                # print a, '*', b, '=', c
                products.add(c)
print sum(products)
