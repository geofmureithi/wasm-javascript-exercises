function fibonacciJS(term) {
    if (term <= 2) return 1;
    return fibonacciJS(term-1) + fibonacciJS(term-2);
}
module.exports = fibonacciJS