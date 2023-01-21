function fibonacci(term) {
  // TODO: Add code here
  if (term <= 2) {
    return 1;
  } else {
    return fibonacci(term - 1) + fibonacci(term - 2);
  }
}

export default fibonacci;
