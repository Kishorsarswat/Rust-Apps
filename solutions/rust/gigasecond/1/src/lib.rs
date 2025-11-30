use time::PrimitiveDateTime as DateTime;
use std::time::Duration;
use core::ops::Add;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    println!("What time is a gigasecond later than {start}");
    dbg!(start.add(Duration::from_secs(1000000000)))
}
