use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::format::{DelayedFormat, StrftimeItems};

pub fn time<'a>(time: i64, offset: i64) -> DelayedFormat<StrftimeItems<'a>> {
    let naive = NaiveDateTime::from_timestamp_opt(time + offset, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
    datetime.format("%H:%M:%S")
}