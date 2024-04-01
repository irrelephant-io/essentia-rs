use essentia_rs::{physics::{Energy, PhaseTransition, Quantity, SpecificHeatCapacity, Temperature}, Builder, Essence, EssenceBuilder, EssenceId};

use super::form::Forms;

pub enum Essences {
    Aqua = 1,
    Pyroflux = 2,
    Heatstone = 3,
    Cryodust = 4,
    Inertia = 5,
    Vitae = 6
}

impl Into<EssenceId> for Essences {
    fn into(self) -> EssenceId {
        (self as u16).into()
    }
}

pub fn create_essences() -> Vec<Essence> {
    Vec::from([
        EssenceBuilder::default()
            .with_name("Aqua")
            .with_custom_id(Essences::Aqua.into())
            .with_specific_heat_capacity(SpecificHeatCapacity::from(4))
            .with_phase_transitions(|builder| {
                builder.add_transition(PhaseTransition {
                    threshold: Temperature::from(100),
                    joules_per_mol: Energy::from(12),
                    left_form_id: Forms::Liquid.into(),
                    right_form_id: Forms::Gas.into()
                });

                builder.add_transition(PhaseTransition {
                    threshold: Temperature::from(0),
                    joules_per_mol: Energy::from(8),
                    left_form_id: Forms::Crystalline.into(),
                    right_form_id: Forms::Liquid.into()
                });
            })
            .with_solubility(|builder| {
                builder
                    .solvent()
                    .with_saturation_limit(Quantity::default())
                    .build()
            })
            .build(),

        EssenceBuilder::default()
            .with_name("Vitae")
            .with_custom_id(Essences::Vitae.into())
            .with_solubility(|builder| {
                builder
                    .solute()
                    .build()
            })
            .build(),

        EssenceBuilder::default()
            .with_name("Pyroflux")
            .with_custom_id(Essences::Pyroflux.into())
            .build(),

        EssenceBuilder::default()
            .with_name("Heatstone")
            .with_custom_id(Essences::Heatstone.into())
            .with_specific_heat_capacity(SpecificHeatCapacity { joule_mol_per_degree: 10 })
            .build(),

        EssenceBuilder::default()
            .with_name("Cryodust")
            .with_custom_id(Essences::Cryodust.into())
            .with_specific_heat_capacity(SpecificHeatCapacity { joule_mol_per_degree: 5 })
            .build(),

        EssenceBuilder::default()
            .with_name("Inertia")
            .with_custom_id(Essences::Inertia.into())
            .with_specific_heat_capacity(SpecificHeatCapacity { joule_mol_per_degree: 0})
            .build()
    ])
}