use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(term: u32) -> u32 {
    if term <= 2 {
        return 1;
    };
    fibonacci(term - 1) + fibonacci(term - 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(10), 55);
    }
}
