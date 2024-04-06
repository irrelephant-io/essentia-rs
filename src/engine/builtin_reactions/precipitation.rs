use crate::physics::{Quantity, Solubility};
use crate::reaction::{Reaction, Product};
use crate::engine::ReactionContext;
use crate::Substance;

pub struct Precipitation;

impl Reaction for Precipitation {
    fn react(
        &self,
        context: &ReactionContext
    ) -> Vec::<Product> {
        context.engine
            .iter_solvents()
            .filter_map(|(solvent, solubility)| {
                let to_precipitate = solubility.get_saturation_percent(solvent) - 1.0;
                if to_precipitate > 0.0 {
                    let absolute_to_precipitate = to_precipitate * solubility.get_saturation_limit(solvent);
                    if absolute_to_precipitate > Quantity::none() {
                        return self.precipitate(context, solvent, absolute_to_precipitate);
                    }
                }

                return None;
            })
            .flat_map(|s| s)
            .collect::<Vec<_>>()
    }

    // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 { u8::MAX - 1 }
}

impl Precipitation {
    fn precipitate(&self, context: &ReactionContext, solvent: &Substance, to_precipitate: Quantity) -> Option<Vec<Product>> {
        if let Substance::Solution(_, _, solutes) = solvent {
            let total_weight = solutes
                .iter()
                .filter_map(|solute| {
                    let essence = context.engine
                        .get_essence(solute.essence_id)
                        .expect("Essence not found");

                    if let Solubility::Solute(_, weight) = essence.solubility.expect("Essence is insoluble") {
                        Some(weight * solute.quantity)
                    } else {
                        None
                    }
                })
                .sum::<Quantity>();

            let mut products = vec![];
            for solute in solutes {
                let essence = context.engine
                    .get_essence(solute.essence_id)
                    .expect("Essence not found");

                if let Solubility::Solute(precipitate_form, weight) = essence.solubility.expect("Essence is insoluble") {
                    let precipitation_ratio = (weight * solute.quantity).mmol as f32 / total_weight.mmol as f32;
                    products.push(Product::Precipitate(
                        solute.essence_id,
                        precipitate_form,
                        solvent.get_substance(),
                        to_precipitate * precipitation_ratio
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
    use crate::{engine::{Essentia, EssentiaBuilder, ReactionContext}, physics::Quantity, reaction::{Product, Reaction}, Builder, EssenceBuilder, EssenceId, Form, FormId, SubstanceBuilder};

    use super::Precipitation;

    enum Forms {
        Liquid = 1,
        Solid = 2
    }

    impl Into<FormId> for Forms {
        fn into(self) -> FormId {
            (self as u16).into()
        }
    }

    enum Essences {
        Water = 1,
        Salt = 2,
        Sugar = 3
    }

    impl Into<EssenceId> for Essences {
        fn into(self) -> EssenceId {
            (self as u16).into()
        }
    }

    fn build_ctx(engine: & Essentia) -> ReactionContext {
        ReactionContext {
            engine: &engine,
            pending_products: vec![]
        }
    }

    fn build_engine() -> Essentia {
        EssentiaBuilder::new()
            .register_form(Form::new_with_id(Forms::Liquid.into(), "Liquid"))
            .register_form(Form::new_with_id(Forms::Solid.into(), "Solud"))
            .register_essence(
                EssenceBuilder::default()
                    .with_custom_id(Essences::Salt.into())
                    .with_solubility(|builder|
                        builder
                            .is_soluble()
                            .when_in_form(Forms::Solid.into())
                            .build()
                    )
                    .build()
            )
            .register_essence(
                EssenceBuilder::default()
                    .with_custom_id(Essences::Water.into())
                    .with_solubility(|builder| 
                        builder
                            .is_solvent()
                            .when_in_form(Forms::Liquid.into())
                            .build()
                    )
                    .build()
            )
            .build()
    }

    #[test]
    pub fn no_precipitation_when_not_saturated() {
        let precipitation = Precipitation {};
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
            .with_solute(solute, Quantity::from(900))
            .build();

        engine.add_substance(solvent);
        let context = build_ctx(&engine);
        let result = precipitation.react(&context);
        assert_eq!(result.len(), 0)
    }

    #[test]
    pub fn precipitates_when_over_saturation_limit() {
        let precipitation = Precipitation {};
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
            .with_solute(solute, Quantity::from(1100))
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
            assert_eq!(qty, Quantity::from(100));
        } else {
            panic!("Idk whats going on");
        }
    }
}
