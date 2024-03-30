use std::sync::atomic::{AtomicU16, Ordering};

use super::physics::SpecificHeatCapacity;

static ESSENCE_COUNTER: AtomicU16 = AtomicU16::new(0);

pub struct Essence {
    pub id: u16,
    pub name: String,

    pub heat_capacity: SpecificHeatCapacity
}

impl Essence {
    pub fn new(name: &str) -> Self {
        Essence {
            id: ESSENCE_COUNTER.fetch_add(1, Ordering::SeqCst),
            name: String::from(name),
            heat_capacity: SpecificHeatCapacity::default()
        }
    }

    pub fn new_with_id(id: u16, name: &str) -> Self {
        ESSENCE_COUNTER.fetch_update(
            Ordering::SeqCst,
            Ordering::SeqCst,
            |current| { Some(u16::max(current, id)) }
        ).unwrap();

        Essence {
            id,
            name: String::from(name),
            heat_capacity: SpecificHeatCapacity::default()
        }
    }
}