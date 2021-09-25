use std::fmt;
use std::cmp;

#[derive(fmt::Debug)]
pub struct Clock {
   hours: i32,
   minutes: i32,
}

impl Clock {

    // If minutes are >= 60 add time to hours
    // If minutes are < 0 remove time from hours
    fn minutes_delta(minutes: i32) -> (i32, i32) {
        let mut new_minutes = minutes % 60;
        let mut adj_hours = minutes / 60;
        if minutes < 0 {
            adj_hours = minutes / 60;
            if minutes.abs() % 60 > 0 {
                adj_hours-=1;
            }
            new_minutes = (60 + minutes % 60 ) % 60;
        }
        (adj_hours, new_minutes)
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = hours;
        let mut new_minutes = minutes % 60;
        let mut adj_hours = minutes / 60;
        if minutes < 0 {
            adj_hours = minutes / 60;
            if minutes.abs() % 60 > 0 {
                adj_hours-=1;
            }
            new_minutes = (60 + minutes % 60 ) % 60;
        }
        if hours + adj_hours < 0 {
            new_hours = 24 - (hours + adj_hours).abs() % 24;
        }
        else{
            new_hours = (hours + adj_hours) % 24;
        }
        Clock{hours:(new_hours) % 24,minutes: new_minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
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
