use crate::{Builder, FormId};

use super::Quantity;

#[derive(Clone, Copy)]
pub enum Solubility {
    Solvent(FormId, Quantity),
    Solute(FormId, Quantity)
}

#[derive(Default)]
pub struct SolubilityBuilder;

impl SolubilityBuilder {
    pub fn is_solvent(self) -> SolventBuilder {
        SolventBuilder::default()
    }

    pub fn is_solute(self) -> SoluteBuilder {
        SoluteBuilder::default()
    }
}

#[derive(Default)]
pub struct SolventBuilder {
    saturation_limit: Option<Quantity>,
    form_id: Option<FormId>
}

impl Builder<Solubility> for SolventBuilder {
    fn build(&self) -> Solubility {
        let limit = self.saturation_limit.unwrap_or_default();
        if let Some(form_id) = self.form_id {
            Solubility::Solvent(form_id, limit)
        } else {
            panic!("Solvent must define a form in which it acts as solvent");
        }
    }
}

impl SolventBuilder {
    pub fn with_saturation_limit(mut self, per_mol: Quantity) -> Self {
        self.saturation_limit = Some(per_mol);
        self
    }

    pub fn when_in_form(mut self, form_id: FormId) -> Self {
        self.form_id = Some(form_id);
        self
    }
}

#[derive(Default)]
pub struct SoluteBuilder {
    weight: Option<Quantity>,
    form_id: Option<FormId>
}

impl Builder<Solubility> for SoluteBuilder {
    fn build(&self) -> Solubility {
        if self.form_id.is_none() {
            panic!("Must specify solute form");
        }
        let weight = self.weight.unwrap_or_default();
        Solubility::Solute(self.form_id.unwrap(), weight)
    }
}

impl SoluteBuilder {
    pub fn with_weight(mut self, per_mol: Quantity) -> Self {
        self.weight = Some(per_mol);
        self
    }

    pub fn soluble_in_form(mut self, form_id: FormId) -> Self {
        self.form_id = Some(form_id);
        self
    }
}
