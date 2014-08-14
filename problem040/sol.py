def bruteForce(n):
    string = ''
    crntS  = 0

    crntN = 1
    result = 1
    while crntN <= n:
        if crntN < len(string):
            result *= int(string[crntN])
        else:
            while crntN >= len(string):
                string += str(crntS)
                crntS += 1
            result *= int(string[crntN])
        crntN *= 10

    print result

bruteForce(1000000)
