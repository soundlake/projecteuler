"""
class Date:
    arrWD = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
    arrMN = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
    arrMD = [   31,    28,    31,    30,    31,    30,    31,    31,    30,    31,    30,    31]

    def __init__(self, string):
        self.date = int(string.split(' ')[0])
        self.month = self.arrMN.index(string.split(' ')[1])
        self.year = int(string.split(' ')[2])
    def __str__(self):
        return str(self.date) + ' ' + self.arrMN[self.month] + ' ' + str(self.year)

    # properties
    @property
    def monthName(self):
        return arrMN[self.month]
    @property
    def daysOfYear(self):
        days = self.date    # date
        days += sum(arrMD[:self.month-1])   # months
        if self.month > 1 and Date.isLeapYear(self.month):
            days += 1 # check if contains Feb of a leap year
        return days
    @property
    def totalDays(self):
        return self.__class__.daysBetweenYears(self.__class__.dayOne().year, self.year)
    @property
    def weekday(self):
        # TODO
        return

    # static methods
    @classmethod
    def dayOne(cls):
        return cls('1 Jan 1900')
    @staticmethod
    def isLeapYear(year):
        return year % 400 == 0 or year % 100 != 0 and year % 4 == 0
    @classmethod
    def numOfLeapYear(cls, a, b):
        num = 0
        for y in range(a, b):
            num += int(cls.isLeapYear(y))
        return num
    @classmethod
    def daysBetweenYears(cls, a, b):
        if (a > b):
            raise TypeError('First parameter should be greater than or equal to second parameter')
        return 365 * (b - a) + cls.numOfLeapYear(a, b)

    # methods
    def sub(self, d):
        if d.__class__.__name__ == 'Date' and self >= d:
            days = self.date - d.date
            # TODO
            return

    # operator overloading - logical
    def __lt__(self, d):
        return self.year < d.year and self.month < d.month and self.date < d.date
    def __le__(self, d):
        return self.__eq__(d) or self.__lt__(d)
    def __eq__(self, d):
        return self.year == d.year and self.month == d.month and self.date == d.date
    def __ne__(self, d):
        return not self.__eq__(d)
    def __ge__(self, d):
        return not self.__lt__(d)
    def __gt__(self, d):
        return not self.__le__(d)
"""



def isLeapYear(year):
    return year % 400 == 0 or year % 100 != 0 and year % 4 == 0

md = [31,28,31,30,31,30,31,31,30,31,30,31]
wd = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
w = 1
answer = 0
for y in range(1901, 2001):
    for m in range(12):
        if w == 0:
            answer += 1
            print y, m+1, wd[w]
        w += md[m] + int(isLeapYear(y) and m == 1)
        w %= 7

print answer
