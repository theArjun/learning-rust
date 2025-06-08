use chrono::prelude::*;
use polars::prelude::*;

fn sample_df() {
    let df: DataFrame =
        df!( // If we want to mutate the DF, use mut.
            "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
            "birthdate" => [
                NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
                NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
                NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
            ],
            "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
            "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
        )
        .unwrap();
    println!("{}", df);
}

fn main() {
    sample_df();
}
