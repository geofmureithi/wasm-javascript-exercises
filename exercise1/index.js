const  fibonacci = require( './js/index')
// const  {fibonacci}= require( './rust/pkg')


console.time("js")
fibonacci(68);
console.timeEnd("js")


// console.time("fibonacci rust");
// fibonacci_rust(68);
// console.timeEnd("fibonacci rust");


