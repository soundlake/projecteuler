def isPalindrome(n):
    return str(n) == str(n)[::-1]



# main logic
palindromes = []
for a in range(100, 1000):
    for b in range(a, 1000):
        if isPalindrome(a * b):
            palindromes.append([a, b, a * b])
print max(palindromes, key = lambda e: e[2])
