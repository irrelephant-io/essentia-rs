use crate::{physics::PhaseGraph, Substance, SubstanceData};

impl super::Essentia {

    fn map_substance<'a>(&self, substance: &'a Substance) -> Vec<&'a SubstanceData> {
        match substance {
            Substance::Normal(n) => vec![n],
            Substance::Solution(n, s) => vec![n, s]
        }
    }

    pub fn get_all(&self) -> impl Iterator<Item = &SubstanceData> {
        self.substances
            .iter()
            .flat_map(|s| self.map_substance(s))

    }

    pub fn get_with_phase_graphs(&self) -> impl Iterator<Item = (&SubstanceData, &PhaseGraph)> {
        self.substances
            .iter()
            .filter_map(|substance| {
                if let Substance::Normal(sd) = substance {
                    let essence = self.get_essence(sd.essence_id).unwrap();
                    if let Some(phase_graph)  = &essence.phase_graph {
                        return Some((sd, phase_graph));
                    }
                }

                // Dissolved substances are not affected by transitions
                None
            })
    }

    pub fn get_of_essense(&self, essence_id: u16) -> impl Iterator<Item = &SubstanceData> {
        self
            .get_all()
            .filter(move |sd| sd.essence_id == essence_id)
    }

    pub fn get_of_form(&self, form_id: u16) -> impl Iterator<Item = &SubstanceData> {
        self
            .get_all()
            .filter(move |sd| sd.form_id == form_id)
    }
}