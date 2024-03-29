#[derive(Copy, Clone)]
pub struct Quantity {
    pub mol: u16
}

impl Quantity {
    pub fn default() -> Self {
        Quantity { mol: 1 }
    }

    pub fn new(mol: u16) -> Self {
        Quantity { mol }
    }
}