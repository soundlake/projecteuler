use chrono::{DateTime, Months};
use chrono::prelude::*;

fn get_answer(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> u32 {
    let one_month: Months = Months::new(1);
    let mut today = start.clone();
    let mut count: u32 = 0;

    while today <= end {
        if today.weekday() == Weekday::Sun {
            count += 1;
        }
        today = today.checked_add_months(one_month).unwrap();
    }

    count
}

fn main() {
    let start = "1901-01-01T00:00:00Z";
    let end  = "2000-12-31T23:59:59Z";

    let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(start).unwrap();
    let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(end).unwrap();

    println!("The answer is: {}", get_answer(start, end));
}
