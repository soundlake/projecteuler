def tInv(t):
    '''
 2t = n^2 + n
  0 = n^2 + n - 2t
  n = (-1 + sqrt(1+8t)) / 2
    '''
    return (-1 + (1+8*t)**.5)/2
def isT(t):
    n = tInv(t)
    return n == int(n)



def pInv(p):
    '''
 2p = 3n^2 - n
  0 = 3n^2 - n - 2p
  n = (1 + sqrt(1 + 24p)) / 6
    '''
    return (1 + (1+24*p)**.5)/6
def isP(p):
    n = pInv(p)
    return n == int(n)



def h(n):
    return 2*n**2 - n



n = 143 + 1
while True:
    hn = h(n)
    if isP(hn) and isT(hn):
        print hn
        break
    n += 1
