const fibonacci = require('./index.js');
const fibonacciJS = require('../js/index.js');


function usingJS(n) {
    const start = performance.now();
    fibonacciJS(n);
    const end = performance.now();
    console.log(end - start);
}

 function usingWebAssembly(n) {
    const start = performance.now();
    fibonacci(n);
    const end = performance.now();
    console.log(end - start);
}

console.log(`Running fibonacci(34) using pure JS: `);
usingJS(34);
console.log(`Running fibonacci(34) using Web Assembly: `);
usingWebAssembly(34); 
