// Use if, else, and loops (loop, while, for) to control program flow:
pub fn learn_control_flow() {
    let x = 10;

    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is 5 or less");
    }

    for i in 0..5 {
        println!("{}", i);
    }
}

pub fn is_number_odd() {
    let x = 23;
    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }
}

pub fn is_number_prime(num: i32) -> bool {
    let mut i = 1; // If we initialize with 0 here, we need to handle case explicitly.
    while i <= num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    learn_control_flow();
    is_number_odd();

    let num = 29;
    if is_number_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}