mod algo;
use algo::bubble_sort;

fn main() {
    let mut numbers: Vec<i32> = vec![5, 2, 1, 5, 1, 5, 6, 1, 19, -10];
    println!("Before sorting: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
