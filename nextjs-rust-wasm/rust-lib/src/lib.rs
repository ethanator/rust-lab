use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(1, 1);
        assert_eq!(result, 2);
        let result = add(2, 2);
        assert_eq!(result, 4);
        let result = add(3, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn fibonacci_works() {
        let result = fibonacci(0);
        assert_eq!(result, 0);
        let result = fibonacci(1);
        assert_eq!(result, 1);
        let result = fibonacci(2);
        assert_eq!(result, 1);
        let result = fibonacci(3);
        assert_eq!(result, 2);
        let result = fibonacci(4);
        assert_eq!(result, 3);
        let result = fibonacci(5);
        assert_eq!(result, 5);
        let result = fibonacci(6);
        assert_eq!(result, 8);
    }
}
