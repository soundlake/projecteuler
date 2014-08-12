def distinctCombination(almt, blmt):
    s = set([])
    for a in range(2, almt + 1):
        for b in range(2, blmt + 1):
            s.add(a ** b)
    return len(s)

print distinctCombination(100, 100)
#print distinctCombination(5, 5)
