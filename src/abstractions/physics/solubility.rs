use crate::Builder;

use super::Quantity;

#[derive(Clone, Copy)]
pub enum Solubility {
    Solvent(Quantity),
    Solute(Quantity)
}

#[derive(Default)]
pub struct SolubilityBuilder;

impl SolubilityBuilder {
    pub fn solvent(self) -> SolventBuilder {
        SolventBuilder::default()
    }

    pub fn solute(self) -> SoluteBuilder {
        SoluteBuilder::default()
    }
}

#[derive(Default)]
pub struct SolventBuilder {
    saturation_limit: Option<Quantity>
}

impl Builder<Solubility> for SolventBuilder {
    fn build(&self) -> Solubility {
        let limit = self.saturation_limit.unwrap_or_default();
        Solubility::Solvent(limit)
    }
}

impl SolventBuilder {
    pub fn with_saturation_limit(mut self, per_mol: Quantity) -> Self {
        self.saturation_limit = Some(per_mol);
        self
    }
}

#[derive(Default)]
pub struct SoluteBuilder {
    weight: Option<Quantity>
}

impl Builder<Solubility> for SoluteBuilder {
    fn build(&self) -> Solubility {
        let weight = self.weight.unwrap_or_default();
        Solubility::Solute(weight)
    }
}

impl SoluteBuilder {
    pub fn with_weight(mut self, per_mol: Quantity) -> Self {
        self.weight = Some(per_mol);
        self
    }
}
