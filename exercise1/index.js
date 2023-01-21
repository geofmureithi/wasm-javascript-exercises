const fib_js = require("./js")
const fib_rs = require("./rust/pkg/rust_fibonacci_bg") 
console.time("jsss")
fib_js(100)
console.timeEnd()

console.time("rs")
fib_rs(100)
console.timeEnd()