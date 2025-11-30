const MINUTES_IN_A_DAY: i32 = 1440;

#[derive(PartialEq, Debug)]
pub struct Clock{
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        println!("Constructing a new Clock from {hours} hours and {minutes} minutes");
        let mut minutes = hours * 60 + minutes;
        minutes = minutes.rem_euclid(MINUTES_IN_A_DAY);
        Self{minutes: minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("Add {minutes} minutes to existing Clock time");
        let mut minutes = self.minutes + minutes;
        minutes = minutes.rem_euclid(MINUTES_IN_A_DAY);
        Self{minutes: minutes}
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        let hours = self.minutes/60;
        let minutes = self.minutes%60;
        format!("{:02}:{:02}",hours,minutes)
    }
}
