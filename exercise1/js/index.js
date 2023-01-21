function fibonacci(term) {
    // TODO: Add code here
    let n1 = 0, n2 = 1, nextTerm;
    nextTerm = n1 + n2;

    while (nextTerm <= term) {

        // print the next term
        console.log(nextTerm);

        n1 = n2;
        n2 = nextTerm;
        nextTerm = n1 + n2;
    }
}

export default fibonacci