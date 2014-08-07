def factorial(n):
    return reduce(lambda x, y: x * y, range(1, n + 1), 1)

def permutation(n, r):
    return reduce(lambda x, y: x * y, range(n - r + 1, n + 1), 1)

def combination(n, r):
    return permutation(n, r) / factorial(r)



# main
print combination(40, 20)
