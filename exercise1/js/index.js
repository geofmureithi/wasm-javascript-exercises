// program to display fibonacci sequence using recursion
function fibonacci(term) {
    if(term < 2) {
        return term;
    }
    else {
        return fibonacci(term-1) + fibonacci(term - 2);
    }
}



export default fibonacci