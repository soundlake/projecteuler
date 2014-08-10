import re

f = open('names.txt', 'r')
raw = f.read()
f.close()

arr = sorted(filter(lambda x: len(x) > 0, re.split('"|,', raw)))
answer = 0
for i in range(len(arr)):
    answer += sum(map(lambda x: ord(x) - ord('A') + 1, list(arr[i]))) * (i+1)
print answer
