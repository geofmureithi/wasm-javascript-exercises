function fibonacci(term) {
    let a = 0, b = 1, result = [0, 1];
    for (let i = 2; i <= term; i++) {
        let c = a + b;
        result.push(c);
        a = b;
        b = c;
    }
    return result;
}

export default fibonacci