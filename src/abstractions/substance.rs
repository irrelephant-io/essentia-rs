use super::quantity::Quantity;


pub struct SubstanceData {
    pub essence_id: u16,
    pub form_id: u16,
    pub quantity: Quantity
}

pub enum Substance {
    // Regular substance
    Normal(SubstanceData),

    // Solution of a different substance
    Solution(SubstanceData, SubstanceData)
}