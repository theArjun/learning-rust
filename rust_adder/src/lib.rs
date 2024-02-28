#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn subtract_numbers(a: i32, b: i32) -> i32 {
    a - b
}
