use super::Temperature;
use super::Time;

pub struct Environment {
    pub temperature: Temperature,
    pub time: Time
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            temperature: Temperature::new(),
            time: Time::new()
        }
    }
}