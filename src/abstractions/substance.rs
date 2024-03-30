use crate::{abstractions::physics::Quantity, engine::Essentia, Essence, Form};

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

pub struct SubstanceBuilder<'a> {
    _private_ctor: (),
    pub engine: &'a Essentia,

    essence: Option<&'a Essence>,
    form: Option<&'a Form>,
    quantity: Option<Quantity>,
    solution_essence: Option<&'a Essence>,
    solution_form: Option<&'a Form>,
    solution_quantity: Option<Quantity>
}

impl SubstanceBuilder<'_> {
    pub fn new<'a>(engine: &'a Essentia) -> SubstanceBuilder<'a> {
        SubstanceBuilder {
            _private_ctor: (),
            engine: engine,
            essence: None,
            solution_essence: None,
            form: None,
            solution_form: None,
            quantity: None,
            solution_quantity: None
        }
    }

    fn try_make_substance(
        &self,
        essence: Option<&Essence>,
        form: Option<&Form>,
        qty: Option<Quantity>
    ) -> Option<SubstanceData> {
        if let (Some(essence), Some(form), Some(qty)) = (essence, form, qty) {
            Some(
                SubstanceData {
                    quantity: qty,
                    essence_id: essence.id,
                    form_id: form.id
                }
            )
        } else {
            None
        }
    }

    pub fn with_essence(mut self, essence_id: u16) -> Self {
        self.essence = self.engine.get_essence(essence_id);
        self
    }

    pub fn with_form(mut self, form_id: u16) -> Self {
        self.form = self.engine.get_form(form_id);
        self
    }

    pub fn with_quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn build(self) -> Substance {
        match (
            self.try_make_substance(self.essence, self.form, self.quantity),
            self.try_make_substance(self.solution_essence, self.solution_form, self.solution_quantity),
        ) {
            (Some(solvent), Some(solution)) => Substance::Solution(solvent, solution),
            (Some(solvent), None) => Substance::Normal(solvent),
            _ => panic!("Couldn't construct substance.")
        }
    }
}