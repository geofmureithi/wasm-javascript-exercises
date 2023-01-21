function fibonacci(term) {
  let a = 0,
    b = 1,
    c;
  if (term === 0) return a;
  for (let i = 2; i <= term; i++) {
    c = a + b;
    a = b;
    b = c;
  }
  return b;
}

export default fibonacci;
