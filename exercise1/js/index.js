function fibonacci(term) {
    let a = 1;
    let b = 1;

    if (term === 0) {
        return 0;
    }

    for (let i = 2; i < term; i++) {
        const c = a + b;
        a = b;
        b = c;
    }

    

    return b;

}

module.exports = fibonacci