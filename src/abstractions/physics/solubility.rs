use crate::{engine::Essentia, Builder, FormId, Substance};

use super::{quantity::PerMol, Quantity};

#[derive(Clone, Copy)]
pub enum Solubility {
    Solvent(FormId, PerMol),
    Solute(FormId, PerMol)
}

impl Solubility {

    pub fn get_saturation_limit(&self, substance: &Substance) -> Quantity {
        if let Solubility::Solvent(_, limit_per_unit) = *self {
            limit_per_unit * substance.get_quantity()
        } else {
            Quantity::none()
        }
    }

    pub fn get_saturation_percent(&self, engine: &Essentia, solvent: &Substance) -> f32 {
        match solvent {
            Substance::Free(_, _) => 0.0,
            Substance::Solution(_, data, solutes) => {
                solutes
                    .iter()
                    .map(|(&essence_id, &quantity)| {
                        let solute_essence = engine
                            .get_essence(essence_id)
                            .expect("Unknown essence in solution!");

                        if let Some(Solubility::Solute(_, weight)) = solute_essence.solubility {
                            weight * quantity
                        } else {
                            panic!("Non-solute found in solution!")
                        }
                    })
                    .sum::<Quantity>().mmol as f32
                / data.quantity.mmol as f32
            }
        }
    }
}

#[derive(Default)]
pub struct SolubilityBuilder;

impl SolubilityBuilder {
    pub fn is_solvent(self) -> SolventBuilder {
        SolventBuilder::default()
    }

    pub fn is_soluble(self) -> SoluteBuilder {
        SoluteBuilder::default()
    }
}

#[derive(Default)]
pub struct SolventBuilder {
    saturation_limit: Option<PerMol>,
    form_id: Option<FormId>
}

impl Builder<Solubility> for SolventBuilder {
    fn build(&self) -> Solubility {
        Solubility::Solvent(
            self.form_id.expect("Form id is required!"),
            self.saturation_limit.unwrap_or_default()
        )
    }
}

impl SolventBuilder {
    pub fn with_saturation_limit(mut self, per_mol: PerMol) -> Self {
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
    weight: Option<PerMol>,
    form_id: Option<FormId>
}

impl Builder<Solubility> for SoluteBuilder {
    fn build(&self) -> Solubility {
        Solubility::Solute(
            self.form_id.expect("Form id is required!"),
            self.weight.unwrap_or_default()
        )
    }
}

impl SoluteBuilder {
    pub fn with_weight(mut self, per_mol: PerMol) -> Self {
        self.weight = Some(per_mol);
        self
    }

    pub fn when_in_form(mut self, form_id: FormId) -> Self {
        self.form_id = Some(form_id);
        self
    }
}
