use polars::prelude::*;
use polars::series::Series;

pub fn learn_series() {
    let s = Series::new("ints".into(), &[1, 2, 3, 4, 5]);
    println!("{}", s)
}

fn main() {
    learn_series();
}
