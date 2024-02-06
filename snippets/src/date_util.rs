use chrono::{NaiveDate, Duration};

#[no_mangle]
pub extern  "C" fn get_date_ranges(start_date: NaiveDate, end_date: NaiveDate) -> Vec<NaiveDate> {
    let mut date_ranges = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        date_ranges.push(current_date);
        current_date = current_date + Duration::days(1);
    }
    date_ranges
}