coin = (1, 2, 5, 10, 20, 50, 100, 200)
limit = 200
ways = map(lambda x: 0, range(limit + 1))
ways[0] = 1

for c in coin:
    for pay in range(c, limit + 1):
        ways[pay] += ways[pay - c]

print ways[200]
