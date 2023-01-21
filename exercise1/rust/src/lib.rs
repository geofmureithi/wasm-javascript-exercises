use wasm_bindgen::prelude::*;



#[wasm_bindgen]
// pub fn fibonacci(term: u32) -> u32 {
//     // TODO: add logic here
//     term
// }


pub fn fibonacci(term: u32) -> u32 {
    if term < 0 {
        panic!("{} is negative!", term);
    }
    match term {
        0 => panic!("zero is not a right argument to fibonacci()!"),
        1 | 2 => 1,
        3 => 2,
        /*
        50    => 12586269025,
        */
        _ => fibonacci(term - 1) + fibonacci(term - 2),
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