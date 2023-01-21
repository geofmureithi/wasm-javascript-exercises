function fibonacci(term) {
    /// impl this function without using recursion
    if (term <= 0) {
        return 0;
    }

    let a = 0;
    let b = 1;
    let c = 0;

    for (let i = 2; i <= term; i++) {
        c = a + b;
        a = b;
        b = c;
    }

    return c;

}

export default fibonacci