use crate::abstractions::physics::{Temperature, Time};

#[derive(Debug)]
pub struct Environment {
    pub temperature: Temperature,
    pub time: Time,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            temperature: Temperature::default(),
            time: Time::new(),
        }
    }
}
