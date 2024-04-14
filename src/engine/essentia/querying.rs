use crate::{
    abstractions::SubstanceId,
    physics::{PhaseGraph, Solubility},
    EssenceId, FormId, Substance,
};

impl super::Essentia {
    pub fn iter_all(&self) -> impl Iterator<Item = &Substance> {
        self.substances.values()
    }

    pub fn iter_mut_all(&mut self) -> impl Iterator<Item = &mut Substance> {
        self.substances.values_mut()
    }

    pub fn iter_solvents(&self) -> impl Iterator<Item = (&Substance, Solubility)> {
        self.substances.values().filter_map(|substance| {
            match substance {
                Substance::Free(_, data) => self
                    .get_solubility(data.essence_id)
                    .map(|solubility| (substance, solubility))
                    .take_if(|(data, solubility)| {
                        matches!(
                            solubility,
                            Solubility::Solvent(solvent_in_form, _)
                            if data.is_form(*solvent_in_form)
                        )
                    }),
                Substance::Solution(_, data, _) => {
                    self.get_solubility(data.essence_id)
                        .map(|solubility| (substance, solubility))
                    // No need to check solubility as it is already a solution base
                    // and as such - must be a solvent.
                }
            }
        })
    }

    pub fn iter_solutes(&self) -> impl Iterator<Item = (&Substance, Solubility)> {
        self.substances.values().filter_map(|substance| {
            match substance {
                Substance::Free(_, data) => self
                    .get_solubility(data.essence_id)
                    .map(|solubility| (substance, solubility))
                    .take_if(|(data, solubility)| {
                        matches!(
                            solubility,
                            Solubility::Solute(soluble_in_form, _)
                            if data.is_form(*soluble_in_form)
                        )
                    }),
                // Only free substances can be solutes
                _ => None,
            }
        })
    }

    pub fn get_solubility(&self, essence_id: EssenceId) -> Option<Solubility> {
        self.essence_lookup.get(&essence_id).map(|x| x.solubility)?
    }

    pub fn get_with_phase_graphs(&self) -> impl Iterator<Item = (&Substance, &PhaseGraph)> {
        self.substances.values().filter_map(|substance| {
            let essence = self
                .get_essence(substance.get_essence())
                .expect("Couldn't find essence when searching for phase graph!");

            essence.phase_graph.as_ref().map(|graph| (substance, graph))
        })
    }

    pub fn get_substance(&self, substance_id: SubstanceId) -> Option<&Substance> {
        self.substances.get(&substance_id)
    }

    pub fn extract_matching(
        &mut self,
        essence_id: EssenceId,
        form_id: FormId,
    ) -> Option<Substance> {
        self.substances
            .extract_if(|_, substance| {
                substance.get_essence() == essence_id && substance.get_form() == form_id
            })
            .next()
            .map(|opt| opt.1)
    }
}
