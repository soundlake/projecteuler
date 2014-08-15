''' interpreting the properties
    d2d3d4=406 is divisible by 2
        ==> d4 is even
    d3d4d5=063 is divisible by 3
        ==> (d3+d4+d5) % 3 == 0
    d4d5d6=635 is divisible by 5
        ==> d6 is 0 or 5
    d5d6d7=357 is divisible by 7
    d6d7d8=572 is divisible by 11
    d7d8d9=728 is divisible by 13
    d8d9d10=289 is divisible by 17
'''
def bruteForce():
    d = map(lambda x: -1, range(11))

    def rmv(arr, n):
        i = arr.index(n)
        return arr[:i] + arr[i+1:]

    valid = []
    for d6 in [0, 5]:
        src = range(10)
        d[6] = d6
        src6 = rmv(src, d6)
        for d1 in src6:
            d[1] = d1
            src1 = rmv(src6, d1)
            for d2 in src1:
                d[2] = d2
                src2 = rmv(src1, d2)
                for d3 in src2:
                    d[3] = d3
                    src3 = rmv(src2, d3)
                    for d4 in src3:
                        if d4%2: continue
                        d[4] = d4
                        src4 = rmv(src3, d4)
                        for d5 in src4:
                            if (d3*100+d4*10+d5)%3!=0: continue
                            d[5] = d5
                            src5 = rmv(src4, d5)
                            for d7 in src5:
                                if (d5*100+d6*10+d7)%7!=0: continue
                                d[7] = d7
                                src7 = rmv(src5, d7)
                                for d8 in src7:
                                    if (d6*100+d7*10+d8)%11!=0: continue
                                    d[8] = d8
                                    src8 = rmv(src7, d8)
                                    for d9 in src8:
                                        if (d7*100+d8*10+d9)%13!=0: continue
                                        d[9] = d9
                                        d10 = rmv(src8, d9)[0]
                                        if (d8*100+d9*10+d10)%17!=0: continue
                                        d[10] = d10
                                        num = int(reduce(lambda x,y: x+str(y), d[1:], ''))
                                        valid.append(num)
    print sum(valid)
    return valid



print sum(bruteForce())
