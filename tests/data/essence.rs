use essentia_rs::{physics::SpecificHeatCapacity, Essence, EssenceBuilder};

pub enum Essences {
    Pyroflux = 1,
    Heatstone = 2
}

impl Into<u16> for Essences {
    fn into(self) -> u16 {
        self as u16
    }
}

pub fn create_essences() -> Vec<Essence> {
    Vec::from([
        EssenceBuilder::default()
            .with_name("Pyroflux")
            .with_custom_id(Essences::Pyroflux.into())
            .build(),

        EssenceBuilder::default()
            .with_name("Heatstone")
            .with_custom_id(Essences::Heatstone.into())
            .with_specific_heat_capacity(SpecificHeatCapacity { joule_mol_per_degree: 10 })
            .build()
    ])
}