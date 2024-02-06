// This attribute tells Rust to use C naming conventions for the add_numbers function.
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {

    let mut count = 100_000_000;
    let mut result: i32;

    loop {
        result = a + b;
        count -=1;
        if count == 0 {
            break;
        }
    }

    result
}

