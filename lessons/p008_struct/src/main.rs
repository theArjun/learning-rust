pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Usage:
// fn main() {
//     let p = struct_::Point { x: 0, y: 0 };
//     println!("{}", p.x);
//     println!("{}", p.y);
// }

pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

// Usage:
// fn main() {
//     let r = struct_::Rectangle {
//         width: 10,
//         height: 10,
//     };
//     let area = r.width * r.height;
//     println!("The area of the rectangle is {} square pixels.", area);
// }

// Traits define shared behavior:
pub trait Area {
    fn area(&self) -> i32; // Like area for Rectangle
}


// Usage :
// impl Area for Rectangle {
//     fn area(&self) -> i32 {
//         self.width * self.height
//     }
// }
//
fn main() {
    let r = Rectangle {
        width: 10,
        height: 10,
    };
    println!("The area of the rectangle is {} square pixels.", r.area());
}