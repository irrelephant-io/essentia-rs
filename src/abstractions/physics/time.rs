use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Time {
    pub ticks: u32
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSpan {
    pub ticks: u16
}

impl From<u16> for TimeSpan {
    fn from(value: u16) -> Self {
        TimeSpan { ticks: value }
    }
}

impl Time {
    pub fn new() -> Self {
        Time { ticks: 0 }
    }
}


impl Add<TimeSpan> for Time {
    type Output = Self;

    fn add(self, rhs: TimeSpan) -> Self::Output {
        Time {
            ticks: self.ticks + rhs.ticks as u32
        }
    }
}

impl AddAssign<TimeSpan> for Time {
    fn add_assign(&mut self, rhs: TimeSpan) {
        self.ticks += rhs.ticks as u32;
    }
}