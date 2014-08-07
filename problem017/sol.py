# sub module
def get4digits(n):
    l = map(lambda x: int(x), list(str(n)))
    if len(l) == 1:
        return [0, 0, 0] + l
    elif len(l) == 2:
        return [0, 0] + l
    elif len(l) == 3:
        return [0] + l
    return l

ones = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
ten  = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"]
tens = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"]
thds = ["", "OneThousand"]

def getWritten(n):
    [d1, d2, d3, d4] = get4digits(n)
    w = thds[d1] + ones[d2]
    if d2:
        w += "Hundred"
        if d3 or d4:
            w += "And"
    if d3 == 1:
        w += ten[d4]
        return w
    return w + tens[d3] + ones[d4]



nums = [""]

for i in range(1, 1001):
    nums.append(getWritten(i))

print sum(map(lambda x: len(x), nums))
