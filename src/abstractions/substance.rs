use std::{
    collections::HashMap,
    sync::atomic::{AtomicU16, Ordering},
};

use crate::{
    abstractions::physics::Quantity, engine::Essentia, physics::Solubility, EssenceId, FormId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubstanceId {
    id: u16,
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
    pub essence_id: EssenceId,
    pub form_id: FormId,
    pub quantity: Quantity,
}

#[derive(Debug)]
pub enum Substance {
    Free(SubstanceId, SubstanceData),
    Solution(SubstanceId, SubstanceData, HashMap<EssenceId, Quantity>),
}

impl Substance {
    pub fn get_form(&self) -> FormId {
        match self {
            Self::Free(_, data) => data.form_id,
            Self::Solution(_, data, _) => data.form_id,
        }
    }

    pub fn is_form(&self, form_id: FormId) -> bool {
        self.get_form() == form_id
    }

    pub fn get_quantity(&self) -> Quantity {
        match self {
            Self::Free(_, data) => data.quantity,
            Self::Solution(_, data, _) => data.quantity,
        }
    }

    pub fn get_essence(&self) -> EssenceId {
        match self {
            Self::Free(_, data) => data.essence_id,
            Self::Solution(_, data, _) => data.essence_id,
        }
    }

    pub fn is_essence(&self, essence_id: EssenceId) -> bool {
        self.get_essence() == essence_id
    }

    pub fn get_substance(&self) -> SubstanceId {
        match self {
            Self::Free(substance_id, _) => *substance_id,
            Self::Solution(substance_id, _, _) => *substance_id,
        }
    }
}

#[non_exhaustive]
pub struct SubstanceBuilder<'a> {
    engine: &'a Essentia,
}

pub struct NormalSubstanceBuilder<'a> {
    engine: &'a Essentia,
    essence_id: Option<EssenceId>,
    form_id: Option<FormId>,
    quantity: Quantity,
}

pub struct SolutionSubstanceBuilder<'a> {
    engine: &'a Essentia,

    substance_id: Option<SubstanceId>,
    essence_id: Option<EssenceId>,
    form_id: Option<FormId>,
    quantity: Quantity,

    solutes: HashMap<EssenceId, Quantity>,
}

impl Substance {
    pub fn divide(self, quantity: Quantity) -> (Self, Option<Self>) {
        match self {
            Substance::Free(id, mut data) => {
                if data.quantity > quantity {
                    let mut remainder_data = data;
                    data.quantity = quantity;
                    remainder_data.quantity -= quantity;
                    return (
                        Substance::Free(id, data),
                        Some(Substance::Free(id, remainder_data)),
                    );
                }
                (self, None)
            }
            _ => panic!("Can't divide solutions!"),
        }
    }
}

static SUBSTANCE_COUNTER: AtomicU16 = AtomicU16::new(0);

impl<'a> SubstanceBuilder<'a> {
    pub fn new(engine: &'a Essentia) -> Self {
        SubstanceBuilder { engine }
    }

    pub fn is_normal(self) -> NormalSubstanceBuilder<'a> {
        NormalSubstanceBuilder::new(self.engine)
    }

    pub fn is_solution(self) -> SolutionSubstanceBuilder<'a> {
        SolutionSubstanceBuilder::new(self.engine)
    }
}

impl<'a> NormalSubstanceBuilder<'a> {
    fn new(engine: &'a Essentia) -> Self {
        NormalSubstanceBuilder {
            engine,
            essence_id: None,
            form_id: None,
            quantity: Quantity::default(),
        }
    }

    pub fn with_form(mut self, form_id: FormId) -> Self {
        self.form_id = Some(form_id);
        self
    }

    pub fn with_essence(mut self, essence_id: EssenceId) -> Self {
        self.essence_id = Some(essence_id);
        self
    }

    pub fn with_quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn build(self) -> Substance {
        if self.essence_id.is_none() {
            panic!("Must specify essence");
        }
        if self.engine.get_essence(self.essence_id.unwrap()).is_none() {
            panic!("Unknown essence!")
        }
        if self.form_id.is_none() {
            panic!("Must specify form");
        }
        if self.engine.get_form(self.form_id.unwrap()).is_none() {
            panic!("Unknown form!")
        }

        Substance::Free(
            SUBSTANCE_COUNTER.fetch_add(1, Ordering::SeqCst).into(),
            SubstanceData {
                essence_id: self.essence_id.unwrap(),
                form_id: self.form_id.unwrap(),
                quantity: self.quantity,
            },
        )
    }
}

impl<'a> SolutionSubstanceBuilder<'a> {
    pub fn build(self) -> Substance {
        if self.essence_id.is_none() {
            panic!("Must specify essence");
        }
        let essence = self.engine.get_essence(self.essence_id.unwrap());
        if essence.is_none() {
            panic!("Unknown essence!")
        }
        if self.form_id.is_none() {
            panic!("Must specify form");
        }
        let form = self.engine.get_form(self.form_id.unwrap());
        if form.is_none() {
            panic!("Unknown form!")
        }
        if let Some(Solubility::Solvent(solvent_form, _)) = essence.unwrap().solubility {
            if solvent_form != self.form_id.unwrap() {
                panic!("This essence doesn't support being solvent in this form!")
            }
        }

        Substance::Solution(
            self.substance_id
                .unwrap_or(SUBSTANCE_COUNTER.fetch_add(1, Ordering::SeqCst).into()),
            SubstanceData {
                essence_id: self.essence_id.unwrap(),
                form_id: self.form_id.unwrap(),
                quantity: self.quantity,
            },
            self.solutes,
        )
    }

    fn new(engine: &'a Essentia) -> Self {
        SolutionSubstanceBuilder {
            engine,
            substance_id: None,
            essence_id: None,
            form_id: None,
            quantity: Quantity::default(),
            solutes: HashMap::new(),
        }
    }

    pub fn with_base(mut self, substance: Substance) -> Self {
        match substance {
            Substance::Free(substance_id, base) => {
                self.substance_id = Some(substance_id);
                self.essence_id = Some(base.essence_id);
                self.form_id = Some(base.form_id);
                self.quantity = base.quantity;
                self.solutes = HashMap::new();
            }
            Substance::Solution(substance_id, base, existing_solutes) => {
                self.substance_id = Some(substance_id);
                self.essence_id = Some(base.essence_id);
                self.form_id = Some(base.form_id);
                self.quantity = base.quantity;
                self.solutes = existing_solutes;
            }
        }

        self
    }

    pub fn with_id(mut self, substance_id: SubstanceId) -> Self {
        self.substance_id = Some(substance_id);
        self
    }

    pub fn with_form(mut self, form_id: FormId) -> Self {
        self.form_id = Some(form_id);
        self
    }

    pub fn with_essence(mut self, essence_id: EssenceId) -> Self {
        self.essence_id = Some(essence_id);
        self
    }

    pub fn with_quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn with_solute(mut self, solute: Substance, quantity: Quantity) -> Self {
        if let Substance::Free(_, solute) = solute {
            self.solutes
                .entry(solute.essence_id)
                .and_modify(|qty| *qty += quantity)
                .or_insert(quantity);
        }

        self
    }

    pub fn with_solutes(mut self, solutes: HashMap<EssenceId, Quantity>) -> Self {
        for kvp in solutes {
            self.solutes.insert(kvp.0, kvp.1);
        }

        self
    }
}
