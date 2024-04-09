use crate::{
    engine::ReactionContext,
    physics::{Quantity, Solubility},
    reaction::{Product, Reaction},
};

pub struct Dissolution {
    pub optimal_dissolution_speed_percent: u32,
}

impl Default for Dissolution {
    fn default() -> Self {
        Self {
            optimal_dissolution_speed_percent: 10,
        }
    }
}

impl Dissolution {
    // This function is balanced to be in equilbrium with precipitation around saturation = 1
    fn get_dissolution_efficiency(&self, saturation: f32) -> f32 {
        if saturation < 0.8 {
            1.0
        } else if saturation < 1.2 {
            -2.25 * (saturation - 0.8) + 1.0
        } else {
            0.1
        }
    }
}

impl Reaction for Dissolution {
    fn react(&self, context: &ReactionContext) -> Vec<Product> {
        let solvents = context.engine.iter_solvents();
        let solutes = context.engine.iter_solutes();

        let total_weight = solutes
            .map(|(solute, solubility)| match solubility {
                Solubility::Solute(_, weight) => solute.get_quantity() * weight,
                _ => Quantity::none(),
            })
            .sum::<Quantity>();

        let total_saturation_limit = solvents
            .map(|(solvent, solubility)| match solubility {
                Solubility::Solvent(_, limit) => solvent.get_quantity() * limit,
                _ => Quantity::none(),
            })
            .sum::<Quantity>();

        let mut products: Vec<Product> = vec![];

        for (solvent, solvent_solubility) in context.engine.iter_solvents() {
            if let Solubility::Solvent(_, saturation_limit) = solvent_solubility {
                let saturation_ratio = self.get_dissolution_efficiency(
                    solvent_solubility.get_saturation_percent(context.engine, solvent),
                );
                let solvent_saturation_limit = solvent.get_quantity() * saturation_limit.mmol_per;
                let relative_saturation =
                    solvent_saturation_limit.mmol as f32 / total_saturation_limit.mmol as f32;

                for (solute, solute_solubility) in context.engine.iter_solutes() {
                    if let Solubility::Solute(_, weight) = solute_solubility {
                        let solute_weight = solute.get_quantity() * weight.mmol_per;
                        let relative_weight = total_weight.mmol as f32 / solute_weight.mmol as f32;
                        let solubility_ratio = relative_saturation / relative_weight;

                        let total_solubility = solubility_ratio * saturation_ratio;

                        let maximum_dissolve =
                            solvent.get_quantity() * self.optimal_dissolution_speed_percent / 100
                                * total_solubility;
                        products.push(Product::Dissolve(
                            solute.get_essence(),
                            solute.get_form(),
                            solvent.get_substance(),
                            maximum_dissolve,
                        ));
                    }
                }
            }
        }

        products
    }

    // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 {
        u8::MAX - 1
    }
}

#[cfg(test)]
mod test {

    use crate::{
        engine::{Essentia, EssentiaBuilder, ReactionContext},
        physics::{PerMol, Quantity},
        reaction::{Product, Reaction},
        Builder, EssenceBuilder, EssenceId, Form, FormId, SubstanceBuilder,
    };

    use super::Dissolution;

    enum Forms {
        Liquid = 1,
        Solid = 2,
    }

    impl From<Forms> for FormId {
        fn from(val: Forms) -> Self {
            (val as u16).into()
        }
    }

    enum Essences {
        Water = 1,
        Salt = 2,
        Sugar = 3,
        AquaRegia = 4,
    }

    impl From<Essences> for EssenceId {
        fn from(val: Essences) -> Self {
            (val as u16).into()
        }
    }

    fn build_ctx(engine: &Essentia) -> ReactionContext {
        ReactionContext {
            engine: engine,
            pending_products: vec![],
        }
    }

    fn build_engine() -> Essentia {
        EssentiaBuilder::new()
            .register_form(Form::new_with_id(Forms::Liquid.into(), "Liquid"))
            .register_form(Form::new_with_id(Forms::Solid.into(), "Solud"))
            .register_essence(
                EssenceBuilder::default()
                    .with_custom_id(Essences::Salt.into())
                    .with_solubility(|builder| {
                        builder
                            .is_soluble()
                            .when_in_form(Forms::Solid.into())
                            .build()
                    })
                    .build(),
            )
            .register_essence(
                EssenceBuilder::default()
                    .with_custom_id(Essences::Sugar.into())
                    .with_solubility(|builder| {
                        builder
                            .is_soluble()
                            .when_in_form(Forms::Solid.into())
                            .with_weight(PerMol::from(2))
                            .build()
                    })
                    .build(),
            )
            .register_essence(
                EssenceBuilder::default()
                    .with_custom_id(Essences::Water.into())
                    .with_solubility(|builder| {
                        builder
                            .is_solvent()
                            .when_in_form(Forms::Liquid.into())
                            .build()
                    })
                    .build(),
            )
            .register_essence(
                EssenceBuilder::default()
                    .with_custom_id(Essences::AquaRegia.into())
                    .with_solubility(|builder| {
                        builder
                            .is_solvent()
                            .when_in_form(Forms::Liquid.into())
                            .with_saturation_limit(PerMol::from(10))
                            .build()
                    })
                    .build(),
            )
            .build()
    }

    #[test]
    fn dissolves_at_max_efficiency_when_not_saturated() {
        let dissolution = Dissolution::default();
        let mut engine = build_engine();

        let solute = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let solvent = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .build();

        engine.add_substance(solvent);
        engine.add_substance(solute);
        let context = build_ctx(&engine);

        let products = dissolution.react(&context);
        assert_eq!(products.len(), 1);
        if let &Product::Dissolve(eid, fid, _, qty) = products.last().unwrap() {
            assert_eq!(eid, Essences::Salt.into());
            assert_eq!(fid, Forms::Solid.into());
            // We expect this much after 1 tick of efficient dissolution
            assert_eq!(
                qty,
                Quantity::default() * dissolution.optimal_dissolution_speed_percent / 100
            );
        }
    }

    #[test]
    fn dissolves_at_minimum_efficiency_if_solution_is_saturated() {
        let dissolution = Dissolution::default();
        let mut engine = build_engine();

        let solute = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let free_solute = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let solvent = SubstanceBuilder::new(&engine)
            .is_solution()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .with_solute(solute, Quantity::default())
            .build();

        engine.add_substance(solvent);
        engine.add_substance(free_solute);
        let context = build_ctx(&engine);

        let products = dissolution.react(&context);
        assert_eq!(products.len(), 1);
        if let &Product::Dissolve(eid, fid, _, qty) = products.last().unwrap() {
            assert_eq!(eid, Essences::Salt.into());
            assert_eq!(fid, Forms::Solid.into());
            // In this form, we expect dissolution at equilibrium with precipitation
            // which amounts to about of max 55% efficiency
            assert_eq!(
                qty,
                Quantity::default() * 0.55 * dissolution.optimal_dissolution_speed_percent / 100
            );
        }
    }

    #[test]
    fn dissolves_inversly_proportional_to_weight_of_solute() {
        let dissolution = Dissolution::default();
        let mut engine = build_engine();

        let salt = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let sugar = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Sugar.into())
            .with_form(Forms::Solid.into())
            .build();

        let solvent = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .build();

        engine.add_substance(solvent);
        engine.add_substance(salt);
        engine.add_substance(sugar);

        let context = build_ctx(&engine);

        let products = dissolution.react(&context);
        let sugar_dissolution_qty = products
            .iter()
            .find_map(|p| match p {
                &Product::Dissolve(essence_id, _, _, qty)
                    if essence_id == Essences::Sugar.into() =>
                {
                    Some(qty)
                }
                _ => None,
            })
            .expect("Expected sugar to start dissolving!");

        let salt_dissolution_qty = products
            .iter()
            .find_map(|p| match p {
                &Product::Dissolve(essence_id, _, _, qty)
                    if essence_id == Essences::Salt.into() =>
                {
                    Some(qty)
                }
                _ => None,
            })
            .expect("Expected salt to start dissolving!");

        // Since sugar is twice as heavy as salt,
        // salt precipitates at 2x the rate
        assert!(
            (salt_dissolution_qty.mmol as f32 / sugar_dissolution_qty.mmol as f32) - 2.0
                < f32::EPSILON
        )
    }

    #[test]
    fn dissolves_inversely_proportional_to_saturation() {
        let dissolution = Dissolution::default();
        let mut engine = build_engine();

        let solute = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let water = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .build();

        let aqua_regia = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::AquaRegia.into())
            .with_form(Forms::Liquid.into())
            .build();

        let water_id = water.get_substance();
        let aqua_regia_id = aqua_regia.get_substance();

        engine.add_substance(water);
        engine.add_substance(solute);
        engine.add_substance(aqua_regia);

        let context = build_ctx(&engine);

        let products = dissolution.react(&context);
        let aqua_regia_dissolution_qty = products
            .iter()
            .find_map(|p| match p {
                &Product::Dissolve(_, _, substance_id, qty) if substance_id == aqua_regia_id => {
                    Some(qty)
                }
                _ => None,
            })
            .expect("Expected aqua regia to dissolve salt!");

        let water_dissolution_qty = products
            .iter()
            .find_map(|p| match p {
                &Product::Dissolve(_, _, substance_id, qty) if substance_id == water_id => {
                    Some(qty)
                }
                _ => None,
            })
            .expect("Expected water to dissolve salt!");

        // Aqua regia is 10 times more powerful solvent,
        // so we expect the ratio of water:aqua regia
        // dissolution to be 1:10
        assert!(
            (aqua_regia_dissolution_qty.mmol as f32 / water_dissolution_qty.mmol as f32) - 10.0
                < 0.1
        )
    }
}
