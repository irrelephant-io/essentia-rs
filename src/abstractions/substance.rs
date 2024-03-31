use std::sync::atomic::{AtomicU16, Ordering};

use crate::{abstractions::physics::Quantity, engine::Essentia, Essence, Form};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SubstanceData {
    pub substance_id: u16,
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

static SUBSTANCE_COUNTER: AtomicU16 = AtomicU16::new(0);

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
    ) -> Result<SubstanceData, &str> {
        if let Some(essence) = essence {
            if let Some(form) = form {
                if let Some(qty) = qty {
                    Ok(SubstanceData {
                        substance_id: SUBSTANCE_COUNTER.fetch_add(1, Ordering::SeqCst),
                        essence_id: essence.id,
                        form_id: form.id,
                        quantity: qty
                    })
                } else {
                    Err("Quantity must be specified!")
                }
            } else {
                Err("Form must be specified!")
            }
        } else {
            Err("Essence must be specified!")
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
            (Ok(solvent), Ok(solution)) => Substance::Solution(solvent, solution),
            (Ok(solvent), Err(_)) => Substance::Normal(solvent),
            (Err(solvent_error), _) => panic!("{}", solvent_error)
        }
    }
}