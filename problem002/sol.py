# initialize
import json
answer = 0

# read data from the file
try:
    f = open('fibonacci.json', 'r')
    fibArr = json.load(f)
    f.close()
except:
    fibArr = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89]

# define a function that returns fib term
def fib(n):
    while len(fibArr) < n :
        fibArr.append(fibArr[-2] + fibArr[-1])
    return fibArr[n-1]

# main
i = 2
while fib(i) < 4000000000:
    answer += fib(i)
    i += 3
print answer

# write data to the file
f = open('fibonacci.json', 'w')
json.dump(fibArr, f)
f.close()
