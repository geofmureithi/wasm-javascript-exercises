const jsFib = require('./js/index')
const wasmFib = require('./rust/pkg/rust_fibonacci')

function measure(fn) {
  const start = Date.now()
  fn()
  const end = Date.now()
  return end - start
}

console.time("js fibonacci")
measure(() => jsFib(34))
console.timeEnd('js fibonacci')

console.time("wasm fibonacci")
measure(() => wasmFib.fibonacci(34))
console.timeEnd('wasm fibonacci')
