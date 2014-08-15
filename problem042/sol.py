def isWordTn(s):
    def isTn(t):
        '''
          t = n(n+1)/2
        2*t = n**2 + n
          0 = n**2 + n - 2*t
          n = (-1 + sqrt(1**2 + 4*1*2*t))/(2*1)
        '''
        n = (-1 + (1 + 8*t)**.5) / 2
        return n == int(n)

    def wordToT(s):
        return reduce(lambda x,y: x + ord(y) - ord('A') + 1, list(s), 0)

    return isTn(wordToT(s))



# read file
f = open('words.txt', 'r')
src = f.read()
f.close()



# parse the content of the file
from re import split
arr = filter(lambda x: x, split('"|,', src))



# filter through only triagle words
result = filter(isWordTn, arr)



# show result
print len(result)
