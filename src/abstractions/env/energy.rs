#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Temperature {
    kelvin: u16
}

pub struct Energy {
    units: u16
}

impl Temperature {
    pub fn new() -> Self {
        Temperature {
            kelvin: 20
        }
    }

    pub fn add(&mut self, delta: Energy) {
        self.kelvin += delta.units;
    }
}

impl Energy {
    pub fn from(units: u16) -> Self {
        Energy { units }
    }
}