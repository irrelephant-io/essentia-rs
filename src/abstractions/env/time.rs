#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Time {
    ticks: u32
}

impl Time {
    pub fn new() -> Self {
        Time { ticks: 0 }
    }

    pub fn from(ticks: u32) -> Self {
        Time { ticks }
    }

    pub fn advance(self: &mut Self, by: &Time) {
        self.ticks += by.ticks;
    }
}