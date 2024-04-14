use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time {
    pub ticks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSpan {
    pub ticks: u32,
}

impl Default for TimeSpan {
    fn default() -> Self {
        Self { ticks: 1 }
    }
}

impl From<u32> for TimeSpan {
    fn from(value: u32) -> Self {
        TimeSpan { ticks: value }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
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
            ticks: self.ticks + rhs.ticks,
        }
    }
}

impl AddAssign<TimeSpan> for Time {
    fn add_assign(&mut self, rhs: TimeSpan) {
        self.ticks += rhs.ticks;
    }
}
