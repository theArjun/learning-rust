pub mod my_modules {
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

use my_modules::subtract;

fn main() {
    let a = 5;
    let b = 10;
    let result = subtract(a, b);
    println!("The result of {} - {} is {}", a, b, result);
}
