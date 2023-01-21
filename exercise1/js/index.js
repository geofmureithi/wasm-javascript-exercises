function fibonacci(term) {
    // TODO: Add code here

let n1 = 0, n2 = 1, nextTerm;

console.log('Fibonacci Series:');

for (let i = 1; i <= term; i++) {
   console.log(n1);
   nextTerm = n1 + n2;
   n1 = n2;
   n2 = nextTerm;
}

}

fibonacci(10)

