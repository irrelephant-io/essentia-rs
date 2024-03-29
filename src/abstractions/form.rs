use std::sync::atomic::{AtomicU16, Ordering};

static FORM_COUNTER: AtomicU16 = AtomicU16::new(0);

pub struct Form {
    pub id: u16,
    pub name: String
}

impl Form {
    pub fn new(name: &str) -> Self {
        Form {
            id: FORM_COUNTER.fetch_add(1, Ordering::SeqCst),
            name: String::from(name)
        }
    }

    pub fn new_with_id(id: u16, name: &str) -> Self {
        FORM_COUNTER.fetch_update(
            Ordering::SeqCst,
            Ordering::SeqCst,
            |current| { Some(u16::max(current, id)) }
        ).unwrap();

        Form {
            id,
            name: String::from(name)
        }
    }
}