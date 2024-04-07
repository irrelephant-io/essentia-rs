use std::sync::atomic::{AtomicU16, Ordering};

static FORM_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormId {
    id: u16
}

impl From<u16> for FormId {
    fn from(value: u16) -> Self {
        FormId { id: value }
    }
}

impl From<FormId> for u16 {
    fn from(value: FormId) -> Self {
        value.id
    }
}

#[derive(Debug)]
pub struct Form {
    pub id: FormId,
    pub name: String
}

impl Form {
    pub fn new(name: &str) -> Self {
        Form {
            id: FORM_COUNTER.fetch_add(1, Ordering::SeqCst).into(),
            name: String::from(name)
        }
    }

    pub fn new_with_id(id: FormId, name: &str) -> Self {
        FORM_COUNTER.fetch_update(
            Ordering::SeqCst,
            Ordering::SeqCst,
            |current| { Some(u16::max(current, id.into())) }
        ).unwrap();

        Form {
            id,
            name: String::from(name)
        }
    }
}