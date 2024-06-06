use std::fmt::{Debug, Display};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn process_time(hours: i32, minutes: i32) -> (i32,i32) {
        let processed_minutes = minutes.rem_euclid(60);
        let additional_hours = minutes / 60;
        let mut processed_hours = (hours + additional_hours).rem_euclid(24);

        // `additional_hours` is not removing an hour with remaining negative minutes.
        // need to do it manually 
        if minutes % 60 != 0 && minutes.is_negative() {
            processed_hours = (processed_hours - 1).rem_euclid(24);
        }

        (processed_hours, processed_minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (new_hours, new_minutes) = Self::process_time(hours, minutes);

        Clock { hours: new_hours, minutes: new_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (new_hours, new_minutes) = Self::process_time(self.hours, self.minutes + minutes);

        Clock { hours: new_hours, minutes: new_minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours_str = if self.hours < 10 { format!("0{}", self.hours) } else { self.hours.to_string() };
        let minutes_str = if self.minutes < 10 { format!("0{}", self.minutes) } else { self.minutes.to_string() };
        write!(f, "{hours_str}:{minutes_str}")
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Clock").field("hours", &self.hours).field("minutes", &self.minutes).finish()
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
