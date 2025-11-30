use chrono::{NaiveDateTime};


pub fn is_in_range(dt: &NaiveDateTime, start: &NaiveDateTime, end: &NaiveDateTime) -> bool {
    dt >= start && dt <= end
}