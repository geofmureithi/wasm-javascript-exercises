use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(term: u32) -> u32 {
    // TODO: add logic here
    if term <= 0 {
        return 0;
    } else if term == 1 {
        return 1;
    } else {
        return fibonacci(term - 1) + fibonacci(term - 2);
    }
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
