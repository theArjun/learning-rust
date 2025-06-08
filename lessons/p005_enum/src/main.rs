pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Up"),
        _  => println!("Other direction")
    }
}
