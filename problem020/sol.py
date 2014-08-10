print reduce(lambda x, y: int(x) + int(y), list(str(reduce(lambda x, y: x * y, range(1, 100+1), 1))), 0)
