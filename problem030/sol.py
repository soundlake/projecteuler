def sumOf5thPower(n):
    return reduce(lambda x, y: x + int(y)**5, list(str(n)), 0)

ans = []
for n in range(2, 1000 * 1000):
    if n == sumOf5thPower(n):
        ans.append(n)
print 'answer:', sum(ans)
