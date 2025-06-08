pub fn learn_variables() {
    // Variables in Rust are immutable by default. Use mut to make them mutable:
    let x = 5;
    let mut y = 10;
    y = 15;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn main() {
    learn_variables()
}
