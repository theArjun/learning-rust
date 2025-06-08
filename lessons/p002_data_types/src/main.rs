// Rust has scalar types (integers like i32, floats like f64, booleans, characters) and compound types (tuples, arrays):
pub fn learn_data_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let arr: [i32; 4] = [1, 2, 3, 4];

    println!("{:?}", tup);
    println!("{:?}", arr);
}

fn main() {
    learn_data_types();
}
