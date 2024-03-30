use essentia_rs::Form;

pub enum Forms {
    Fluid = 1,
    Salt = 2,
    Gas = 3
}

impl Into<u16> for Forms {
    fn into(self) -> u16 {
        self as u16
    }
}

pub fn create_forms() -> Vec<Form> {
    Vec::from([
        Form::new_with_id(Forms::Fluid as u16, "Fluid"),
        Form::new_with_id(Forms::Salt as u16, "Salt"),
        Form::new_with_id(Forms::Salt as u16, "Gas")
    ])
}