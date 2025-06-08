use std::io;
fn main() {
    let mut string = String::new();
    println!("Enter a string: ");
    io::stdin()
        .read_line(&mut string)
        .expect("Expected user input");
    println!("You have entered this string: {}", string);

    let mut line_input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut line_input)
        .expect("Expected user input");
    let number: i32 = line_input.trim().parse::<i32>().unwrap();
    println!("You have entered this number: {}", number)
}
