use chrono::{Datelike, NaiveDate};
use crate::date_util::get_date_ranges;

mod date_util;

fn main() {

    let start_date = NaiveDate::from_ymd_opt(2023,1,1).expect("Invalid start date");
    let end_date = NaiveDate::from_ymd_opt(2023,2,28).expect("Invalid end date");
    let date_ranges = get_date_ranges(start_date, end_date);

    for date in date_ranges {
        println!("{}", date.weekday())
    }
}