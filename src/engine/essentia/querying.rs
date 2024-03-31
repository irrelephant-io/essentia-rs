use crate::{Substance, SubstanceData};

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

    pub fn get_of_essense(&self, essence_id: u16) -> impl Iterator<Item = &SubstanceData> {
        self
            .get_all()
            .filter(move |&sd| sd.essence_id == essence_id)
    }
}