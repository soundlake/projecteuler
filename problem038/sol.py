def isPandigital(s):
    return '123456789' == reduce(lambda x,y: x+y, sorted(list(s)))

for n in range(9876, 9122, -1):
    if isPandigital(str(n) + str(n*2)):
        print n, (str(n) + str(n*2))
        break
