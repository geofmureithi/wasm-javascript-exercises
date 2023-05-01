// program to display fibonacci sequence using recursion
function fibonacci(term) {
    if(term < 2) {
        return term;
    }
    else {
        return fibonacci(term-1) + fibonacci(term - 2);
    }
}

// take nth term input from the user
const nTerms = prompt('Enter the termber of terms: ');

if(nTerms <=0) {
    console.log('Enter a positive integer.');
}
else {
    for(let i = 0; i < nTerms; i++) {
        console.log(fibonacci(i));
    }
}

export default fibonacci