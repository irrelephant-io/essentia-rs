use essentia_rs::Form;

pub enum Forms {
    Crystalline = 1,
    Liquid = 2,
    Salt = 3,
    Gas = 4
}

impl Into<u16> for Forms {
    fn into(self) -> u16 {
        self as u16
    }
}

pub fn create_forms() -> Vec<Form> {
    Vec::from([
        Form::new_with_id(Forms::Crystalline.into(), "Crystalline"),
        Form::new_with_id(Forms::Liquid as u16, "Liquid"),
        Form::new_with_id(Forms::Salt as u16, "Salt"),
        Form::new_with_id(Forms::Gas as u16, "Gas")
    ])
}