const MINUTES_IN_A_DAY: i32 = 1440;
use std::fmt;
#[derive(PartialEq, Debug)]
pub struct Clock{
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = hours * 60 + minutes;
        minutes = minutes.rem_euclid(MINUTES_IN_A_DAY);
        Self{minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut minutes = self.minutes + minutes;
        minutes = minutes.rem_euclid(MINUTES_IN_A_DAY);
        Self{minutes}
    }
}

// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         let hours = self.minutes/60;
//         let minutes = self.minutes%60;
//         format!("{hours:02}:{minutes:02}")
//     }
// }

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes/60;
        let minutes = self.minutes%60;
        write!(f, "{hours:02}:{minutes:02}")
    }
}
