'''
d = b - a, d is pentagonal
b + a = d + 2a is also pentagonal
'''
def p(n):
    return (3*n**2 - n) / 2
def pInv(p):
    return (1 + (1 + 24*p)**.5) / 6



def isPentagonal(p):
    n = pInv(p)
    return n == int(n)



def generatePentagonalUpto(limit):
    arr = [0]
    for n in range(1, limit+1):
        arr.append(p(n))
    return arr



def findDifUpto(limit):
    for a in range(1, limit):
        A = p(a)
        for b in range(a+1, limit+1):
            B = p(b)
            D = B-A
            if isPentagonal(A+B) and isPentagonal(D):
                sa = 'p('+str(a)+')='+str(A)
                sb = 'p('+str(b)+')='+str(B)
                return sa+', '+sb, 'D='+str(D)



print findDifUpto(10000)
