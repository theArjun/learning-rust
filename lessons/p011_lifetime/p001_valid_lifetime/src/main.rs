// valid lifetime

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let string_1 = String::from("abcd");
    let string_2 = String::from("xyz");

    let result = longest(&string_1, &string_2);
    println!("The longest string is {}", result);
}