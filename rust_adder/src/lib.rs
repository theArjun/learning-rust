pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// This attribute tells Rust to use C naming conventions for the add_numbers function.
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
