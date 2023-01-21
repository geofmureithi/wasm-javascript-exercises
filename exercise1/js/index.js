function fibonacci(term) {
    // TODO: Add code here
    let a = 0, b = 1;
  let fib = [a, b];
  for (let i = 2; i <= term; i++) {
    let c = a + b;
    fib.push(c);
    a = b;
    b = c;
  }
  return fib.pop();
}

//export default fibonacci
let result = fibonacci(9)

console.log(result)