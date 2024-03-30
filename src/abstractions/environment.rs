use crate::abstractions::physics::{Temperature, Time};

pub struct Environment {
    pub temperature: Temperature,
    pub time: Time
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            temperature: Temperature::default(),
            time: Time::new()
        }
    }
}