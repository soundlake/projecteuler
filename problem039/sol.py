def isRT(a, b, c):
    return a**2 + b**2 == c**2



def findRTs(n):
    result = []
    for a in range(1, n/3 + 1):
        for b in range(a, (n-a)/2 + 1):
            c = n - a - b
            if isRT(a, b, c):
                result.append((a, b, c))
    return result



def findMostRTgenIn(limit):
    result = []
    if limit < 12: return result
    for n in range(12, limit + 1):
        RTs = findRTs(n)
        if len(RTs) > 0:
            result.append((n, RTs))
    return result



RTgens = findMostRTgenIn(1000)
answer = max(RTgens, key=lambda x: len(x[1]))
print answer
