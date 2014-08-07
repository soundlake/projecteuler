def findLength(n):
    length = 0
    while n > 1:
        if n % 2:
            n = n * 3 + 1
        else:
            n /= 2
        length += 1
    return length

lens = {}
for i in range(1000 * 1000 + 1):
    lens[i] = findLength(i)

print max(lens.items(), key = lambda x: x[1])
