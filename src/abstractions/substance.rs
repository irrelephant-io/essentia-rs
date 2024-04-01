use std::sync::atomic::{AtomicU16, Ordering};

use crate::{abstractions::physics::Quantity, engine::Essentia, Essence, EssenceId, Form, FormId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubstanceId {
    id: u16
}

impl From<u16> for SubstanceId {
    fn from(value: u16) -> Self {
        SubstanceId { id: value }
    }
}

impl From<SubstanceId> for u16 {
    fn from(value: SubstanceId) -> Self {
        value.id
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SubstanceData {
    pub substance_id: SubstanceId,
    pub essence_id: EssenceId,
    pub form_id: FormId,
    pub quantity: Quantity
}

impl SubstanceData {
    pub fn new(
        essence_id: EssenceId,
        form_id: FormId,
        quantity: Quantity
    ) -> Self {
        SubstanceData {
            substance_id: SUBSTANCE_COUNTER.fetch_add(1, Ordering::SeqCst).into(),
            essence_id,
            form_id,
            quantity
        }
    }
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
                        substance_id: SUBSTANCE_COUNTER.fetch_add(1, Ordering::SeqCst).into(),
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

    pub fn with_essence(mut self, essence_id: EssenceId) -> Self {
        self.essence = self.engine.get_essence(essence_id);
        self
    }

    pub fn with_form(mut self, form_id: FormId) -> Self {
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