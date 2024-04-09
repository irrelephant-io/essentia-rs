use essentia_rs::{Form, FormId};

#[derive(Clone, Copy)]
pub enum Forms {
    Crystalline = 1,
    Liquid = 2,
    Salt = 3,
    Gas = 4,
}

impl Into<FormId> for Forms {
    fn into(self) -> FormId {
        (self as u16).into()
    }
}

pub fn create_forms() -> Vec<Form> {
    Vec::from([
        Form::new_with_id(Forms::Crystalline.into(), "Crystalline"),
        Form::new_with_id(Forms::Liquid.into(), "Liquid"),
        Form::new_with_id(Forms::Salt.into(), "Salt"),
        Form::new_with_id(Forms::Gas.into(), "Gas"),
    ])
}
