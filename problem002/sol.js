console.log((function SumOfEvenFibonacciWithin (n) {
    return (function fibArrUpto() {
        arr = [1, 2, 3, 5]
        while (arr[arr.length-1] < n)
            arr[arr.length] = arr[arr.length-2] + arr[arr.length-1]
        return arr
    })(n)
    .filter(function (el) { return el % 2 == 0; })
    .reduce(function (prev, curr) { return prev + curr; }, 0)
})(4000 * 1000))
