limit = 1000 * 1000
digits = range(10)



def fact(n):
    return reduce(lambda x, y: x * y, range(1, n + 1), 1)



def findNthLexOrder(arr, n):
    if len(arr) == 1:
        return str(arr[0])
    if n == 1:
        return reduce(lambda x, y: x + str(y), arr, '')

    nFact = fact(len(arr)-1)
    idx = 0
    while nFact < n:
        n -= nFact
        idx += 1
    return str(arr[idx]) + findNthLexOrder(arr[:idx] + arr[idx+1:], n)



print findNthLexOrder(digits, limit)
