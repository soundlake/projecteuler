def isPalindromic(s):
    if len(s) == 1: return True
    l = len(s) / 2
    return s[:l] == s[-l:][::-1]

def is10and2(n):
    if isPalindromic(str(n)) and isPalindromic(bin(n)[2:]):
        print n, bin(n)[2:]
        return True
    return False

def find10and2Upto(n):
    return filter(is10and2, range(1, n))

print sum(find10and2Upto(1000*1000))
