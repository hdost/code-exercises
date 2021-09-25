use std::cmp;
use std::fmt;

#[derive(fmt::Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
// If minutes are >= 60 add time to hours
// If minutes are < 0 remove time from hours
fn minutes_delta(minutes: i32) -> (i32, i32) {
    let mut new_minutes = minutes % 60;
    let mut adj_hours = minutes / 60;
    if minutes < 0 {
        adj_hours = minutes / 60;
        if minutes.abs() % 60 > 0 {
            adj_hours -= 1;
        }
        new_minutes = (60 + minutes % 60) % 60;
    }
    (adj_hours, new_minutes)
}
fn hours_delta(hours: i32, hours_delta: i32) -> i32 {
    let new_hours: i32;
    if hours + hours_delta < 0 {
        new_hours = (24 - (hours + hours_delta).abs() % 24) % 24;
    } else {
        new_hours = (hours + hours_delta) % 24;
    }
    new_hours
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (adj_hours, new_minutes) = minutes_delta(minutes);
        Clock {
            hours: hours_delta(hours, adj_hours),
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (adj_hours, new_minutes) = minutes_delta(minutes + self.minutes);
        Clock {
            hours: hours_delta(self.hours, adj_hours),
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
