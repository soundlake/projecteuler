def solve(lmt):
    n = 1
    d = 2
    sum = n

    while n < lmt * lmt:
        for i in range(4):
            n += d
            sum += n
        d += 2

    return sum

print solve(1001)
