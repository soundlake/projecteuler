def fact(n):
    return reduce(lambda x,y: x*y, range(1, n+1), 1)

def isValid(n):
    return n == reduce(lambda x,y: x + fact(int(y)), list(str(n)), 0)

def bruteForce():
    data = []
    for n in range(10, fact(10)):
        if isValid(n):
            data.append(n)
            print n
    return sum(data)

print bruteForce()
