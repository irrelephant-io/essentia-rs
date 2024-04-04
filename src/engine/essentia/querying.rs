use crate::{physics::{PhaseGraph, Solubility}, EssenceId, Substance, SubstanceData};

impl super::Essentia {
    pub fn iter_all(&self) -> impl Iterator<Item = &Substance> {
        self.substances.values()
    }

    pub fn iter_mut_all(&mut self) -> impl Iterator<Item = &mut Substance> {
        self.substances.values_mut()
    }

    pub fn iter_solvents(&self) -> impl Iterator<Item = (&Substance, Solubility)> {
        self.substances
            .values()
            .filter_map(|substance| {
                match substance {
                    Substance::Free(_, data) => {
                        self
                            .get_solubility(data.essence_id)
                            .map(|solubility| (substance, solubility))
                            .take_if(|(data, solubility)| {
                                matches!(
                                    solubility,
                                    Solubility::Solvent(solvent_in_form, _)
                                    if data.is_form(*solvent_in_form)
                                )
                            })
                    },
                    Substance::Solution(_, data, _) => {
                        self
                            .get_solubility(data.essence_id)
                            .map(|solubility| (substance, solubility))
                            // No need to check solubility as it is already a solution base
                            // and as such - must be a solvent.
                    }
                }
            })
    }

    pub fn iter_solutes(&self) -> impl Iterator<Item = (&Substance, Solubility)> {
        self.substances
            .values()
            .filter_map(|substance| {
                match substance {
                    Substance::Free(_, data) => {
                        self
                            .get_solubility(data.essence_id)
                            .map(|solubility| (substance, solubility))
                            .take_if(|(data, solubility)| {
                                matches!(
                                    solubility,
                                    Solubility::Solute(soluble_in_form, _)
                                    if data.is_form(*soluble_in_form)
                                )
                            })
                    },
                    // Only free substances can be solutes
                    _ => None
                }
            })
    }

    pub fn get_solubility(&self, essence_id: EssenceId) -> Option<Solubility> {
        self
            .essence_lookup
            .get(&essence_id)
            .map(|x| x.solubility)?
    }

    pub fn get_with_phase_graphs(&self) -> impl Iterator<Item = (&SubstanceData, &PhaseGraph)> {
        self.substances
            .values()
            .filter_map(|substance| {
                if let Substance::Free(_, sd) = substance {
                    let essence = self.get_essence(sd.essence_id).unwrap();
                    if let Some(phase_graph)  = &essence.phase_graph {
                        return Some((sd, phase_graph));
                    }
                }

                // Dissolved substances are not affected by transitions
                None
            })
    }
}