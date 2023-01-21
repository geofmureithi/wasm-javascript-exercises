use wasm_bindgen::prelude::*;



#[wasm_bindgen]
pub fn fibonacci(term: u32) -> u32 {
   let mut a = 1;
    let mut b = 1;


    for _ in 2..term {
        let c = a + b;
        a = b;
        b = c;
    }

    b
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