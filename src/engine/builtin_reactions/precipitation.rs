use crate::engine::ReactionContext;
use crate::physics::{Quantity, Solubility};
use crate::reaction::{Product, Reaction};
use crate::Substance;

pub struct Precipitation {
    pub optimal_precipitation_speed_percent: u32,
}

impl Default for Precipitation {
    fn default() -> Self {
        Self {
            optimal_precipitation_speed_percent: 10,
        }
    }
}

impl Reaction for Precipitation {
    fn react(&self, context: &ReactionContext) -> Vec<Product> {
        context
            .engine
            .iter_solvents()
            .filter_map(|(solvent, solubility)| {
                let precipitation_efficiency = self.get_precipitation_efficiency(
                    solubility.get_saturation_percent(context.engine, solvent),
                );
                if precipitation_efficiency > 0.0 {
                    let absolute_to_precipitate = precipitation_efficiency
                        * solubility.get_saturation_limit(solvent)
                        * self.optimal_precipitation_speed_percent
                        / 100;

                    if absolute_to_precipitate > Quantity::none() {
                        return self.precipitate(context, solvent, absolute_to_precipitate);
                    }
                }

                None
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 {
        u8::MAX - 1
    }
}

impl Precipitation {
    // This function is balanced to be in equilbrium in dissolution around saturation = 1
    fn get_precipitation_efficiency(&self, saturation: f32) -> f32 {
        if saturation < 0.8 {
            0.1
        } else if saturation < 1.2 {
            2.25 * (saturation - 0.8) + 0.1
        } else {
            1.0
        }
    }

    fn precipitate(
        &self,
        context: &ReactionContext,
        solvent: &Substance,
        to_precipitate: Quantity,
    ) -> Option<Vec<Product>> {
        if let Substance::Solution(_, _, solutes) = solvent {
            let total_weight = solutes
                .iter()
                .filter_map(|(&essence_id, &solute_quantity)| {
                    let essence = context
                        .engine
                        .get_essence(essence_id)
                        .expect("Essence not found");

                    if let Solubility::Solute(_, weight) =
                        essence.solubility.expect("Essence is insoluble")
                    {
                        Some(weight * solute_quantity)
                    } else {
                        None
                    }
                })
                .sum::<Quantity>();

            let mut products = vec![];
            for (&essence_id, &solute_quantity) in solutes {
                let essence = context
                    .engine
                    .get_essence(essence_id)
                    .expect("Essence not found");

                if let Solubility::Solute(precipitate_form, weight) =
                    essence.solubility.expect("Essence is insoluble")
                {
                    let precipitation_ratio =
                        (weight * solute_quantity).mmol as f32 / total_weight.mmol as f32;
                    products.push(Product::Precipitate(
                        essence_id,
                        precipitate_form,
                        solvent.get_substance(),
                        to_precipitate * precipitation_ratio,
                    ))
                }
            }

            Some(products)
        } else {
            None
        }
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

    use super::Precipitation;

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
            .build()
    }

    #[test]
    fn minumum_precipitation_when_not_saturated() {
        let precipitation = Precipitation::default();
        let mut engine = build_engine();

        let solute = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let solvent = SubstanceBuilder::new(&engine)
            .is_solution()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .with_solute(solute, Quantity::from(700))
            .build();

        engine.add_substance(solvent);
        let context = build_ctx(&engine);
        let result = precipitation.react(&context);
        let product = result.last().unwrap();
        if let &Product::Precipitate(_, _, _, qty) = product {
            // In this stage, we expect minimal possible precipitation of 10%
            // since solution is less than 80% saturated
            assert_eq!(
                qty,
                Quantity::default() * 0.1 * precipitation.optimal_precipitation_speed_percent / 100
            );
        } else {
            panic!("Didn't precipitate when expected!");
        }
    }

    #[test]
    fn precipitates_when_over_saturation_limit() {
        let precipitation = Precipitation::default();
        let mut engine = build_engine();

        let solute = SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Salt.into())
            .with_form(Forms::Solid.into())
            .build();

        let solvent = SubstanceBuilder::new(&engine)
            .is_solution()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .with_solute(solute, Quantity::from(1300))
            .build();

        let solvent_id = solvent.get_substance();

        engine.add_substance(solvent);
        let context = build_ctx(&engine);
        let result = precipitation.react(&context);
        assert_eq!(result.len(), 1);
        let product = result.last().unwrap();
        if let &Product::Precipitate(eid, fid, sid, qty) = product {
            assert_eq!(eid, Essences::Salt.into());
            assert_eq!(fid, Forms::Solid.into());
            assert_eq!(sid, solvent_id);
            // Since we are over 20% supersaturated, precipitation happens at
            // maximum possible rate.
            assert_eq!(
                qty,
                Quantity::default() * precipitation.optimal_precipitation_speed_percent / 100
            );
        } else {
            panic!("Weird product detected in precipitation reaction!");
        }
    }

    #[test]
    fn pricipitates_proportional_to_weight() {
        let precipitation = Precipitation::default();
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
            .is_solution()
            .with_essence(Essences::Water.into())
            .with_form(Forms::Liquid.into())
            .with_solute(salt, Quantity::from(1000))
            .with_solute(sugar, Quantity::from(1000))
            .build();

        engine.add_substance(solvent);
        let context = &build_ctx(&engine);
        let products = precipitation.react(context);

        assert_eq!(products.len(), 2);
        let sugar_precipitation_qty = products
            .iter()
            .find_map(|p| match p {
                &Product::Precipitate(essence_id, _, _, qty)
                    if essence_id == Essences::Sugar.into() =>
                {
                    Some(qty)
                }
                _ => None,
            })
            .expect("Expected sugar to start precipitating!");

        let salt_precipitation_qty = products
            .iter()
            .find_map(|p| match p {
                &Product::Precipitate(essence_id, _, _, qty)
                    if essence_id == Essences::Salt.into() =>
                {
                    Some(qty)
                }
                _ => None,
            })
            .expect("Expected salt to start precipitating!");

        assert!(
            (sugar_precipitation_qty.mmol as f32 / salt_precipitation_qty.mmol as f32) - 2.0
                < f32::EPSILON
        )
    }
}
