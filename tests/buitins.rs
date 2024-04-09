use data::form::Forms;
use data::reactions::CryodustChill;
use data::{essence::Essences, reactions::PyroflaxHeat};
use essentia_rs::physics::Power;
use essentia_rs::{
    engine::{Essentia, EssentiaBuilder},
    physics::{Quantity, Rate, Temperature, TimeSpan},
    SubstanceBuilder,
};
use essentia_rs::{Substance, SubstanceId};

pub mod data;

fn setup() -> Essentia {
    // Create engine WITH built-in reactions
    let mut builder = EssentiaBuilder::default();

    builder = data::essence::create_essences()
        .into_iter()
        .fold(builder, |it, e| it.register_essence(e));

    builder = data::form::create_forms()
        .into_iter()
        .fold(builder, |it, e| it.register_form(e));

    builder
        .register_reaction(Box::new(PyroflaxHeat::from(42)))
        .register_reaction(Box::new(CryodustChill::new(Power::from(40), Rate::from(0))))
        .build()
}

fn add_pyroflux(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(engine)
            .is_normal()
            .with_essence(Essences::Pyroflux.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::default() * 10)
            .build(),
    );
}

fn add_cryodust(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(engine)
            .is_normal()
            .with_essence(Essences::Cryodust.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::from(10))
            .build(),
    );
}

fn add_water(engine: &mut Essentia, quantity: Quantity) -> SubstanceId {
    let substance = SubstanceBuilder::new(engine)
        .is_normal()
        .with_essence(Essences::Aqua.into())
        .with_quantity(quantity)
        .with_form(Forms::Liquid.into())
        .build();
    let id = substance.get_substance();
    engine.add_substance(substance);
    id
}

fn add_saline(engine: &mut Essentia, quantity: Quantity) -> SubstanceId {
    let substance = SubstanceBuilder::new(engine)
        .is_normal()
        .with_essence(Essences::Saline.into())
        .with_quantity(quantity)
        .with_form(Forms::Crystalline.into())
        .build();
    let id = substance.get_substance();
    engine.add_substance(substance);
    id
}

fn add_vitae(engine: &mut Essentia, quantity: Quantity) {
    engine.add_substance(
        SubstanceBuilder::new(engine)
            .is_normal()
            .with_essence(Essences::Vitae.into())
            .with_quantity(quantity)
            .with_form(Forms::Crystalline.into())
            .build(),
    );
}

fn get_of_form(engine: &Essentia, form: Forms) -> impl Iterator<Item = &Substance> {
    engine.iter_all().filter(move |s| s.is_form(form.into()))
}

fn get_of_essense(engine: &Essentia, essence: Essences) -> impl Iterator<Item = &Substance> {
    engine
        .iter_all()
        .filter(move |s| s.is_essence(essence.into()))
}

const TRIAL_LIMIT: u32 = 100_000;
const WATER_BOIL_TEMP: Temperature = Temperature { mkelvin: 373_000 };
const WATER_CRYSTALLIZATION_TEMP: Temperature = Temperature { mkelvin: 273_000 };

fn assert_trial_limit(trial: &mut u32) {
    *trial += 1;
    if *trial >= TRIAL_LIMIT {
        panic!("Trial limit reached!");
    }
}

#[test]
fn test_water_evaporation_transitions() {
    let mut engine = setup();
    add_water(&mut engine, Quantity::from(10_000));
    add_pyroflux(&mut engine);

    let mut trial: u32 = 0;
    while engine.environment.temperature < WATER_BOIL_TEMP {
        assert_trial_limit(&mut trial);
        engine.simulate(TimeSpan::default())
    }
    println!("water boiling point reached after {} trials", trial);
    trial = 0;

    while engine.get_form(Forms::Liquid.into()).into_iter().count() == 1 {
        assert_trial_limit(&mut trial);
        engine.simulate(TimeSpan::default());
        let liquid_count = get_of_form(&engine, Forms::Liquid).count();
        if liquid_count == 0 {
            // We have succesfully evaporated all of the water. Now there is only steam!
            break;
        }
        assert_eq!(
            engine.environment.temperature, WATER_BOIL_TEMP,
            "While form transition is happening, all energy goes to transition."
        )
    }

    assert_eq!(engine.get_form(Forms::Gas.into()).into_iter().count(), 1);
}

#[test]
fn test_water_crystalization_transitions() {
    let mut engine = setup();
    add_water(&mut engine, Quantity::from(20));
    add_cryodust(&mut engine);

    let mut trial: u32 = 0;
    while engine.environment.temperature > WATER_CRYSTALLIZATION_TEMP {
        assert_trial_limit(&mut trial);
        engine.simulate(TimeSpan::default())
    }
    println!("water freezing point reached after {} trials", trial);
    trial = 0;

    while engine.get_form(Forms::Liquid.into()).into_iter().count() == 1 {
        assert_trial_limit(&mut trial);
        engine.simulate(TimeSpan::default());
        let liquid_count = get_of_form(&engine, Forms::Liquid).count();
        if liquid_count == 0 {
            // We have succesfully evaporated all of the water. Now there is only steam!
            break;
        }
        assert_eq!(
            engine.environment.temperature, WATER_CRYSTALLIZATION_TEMP,
            "While form transition is happening, all energy goes to transition."
        )
    }

    assert_eq!(
        engine
            .get_form(Forms::Crystalline.into())
            .into_iter()
            .count(),
        1
    );
}

#[test]
fn test_solution_in_water() {
    let mut engine = setup();
    let starting_vitae = Quantity::from(50000);
    add_water(&mut engine, Quantity::from(20000));
    add_vitae(&mut engine, starting_vitae);

    engine.simulate(TimeSpan::default());

    // Vitae should have started dissolving in water, so now there is free vitae
    // and also a solution in water
    let substances = engine.iter_all().collect::<Vec<_>>();
    assert_eq!(substances.len(), 2);

    let total_vitae = substances
        .iter()
        .filter_map(|substance| {
            match &substance {
                Substance::Free(_, data) => {
                    if data.essence_id == Essences::Vitae.into() {
                        assert!(data.quantity < starting_vitae, "Some vitae is supposed to have dissolved!");
                        return Some(data.quantity);
                    }
                    panic!("Free non-vitae? this is not expected. All of the water should have become a solution!")
                },
                Substance::Solution(_, data, solutes) => {
                    assert_eq!(data.essence_id, Essences::Aqua.into(), "There is just one solvent - water!");
                    assert_eq!(1, solutes.len(), "Only one solute should be present!");
                    let &vitae_solute_qty = solutes.get(&Essences::Vitae.into()).unwrap();
                    assert!(vitae_solute_qty < starting_vitae);
                    Some(vitae_solute_qty)
                }
            }
        })
        .sum::<Quantity>();
    assert_eq!(total_vitae, starting_vitae);

    let mut trial: u32 = 0;
    while get_of_essense(&engine, Essences::Vitae).count() != 1 {
        assert_trial_limit(&mut trial);
        engine.simulate(TimeSpan::default());
    }
}

#[test]
fn test_dissolution_equilibrium() {
    let mut engine = setup();

    let saline_qty = Quantity::default();
    let vitae_qty = Quantity::from(500);
    // We add just enough vitae to completely saturate the water solution
    add_water(&mut engine, Default::default());
    add_vitae(&mut engine, vitae_qty);

    let mut trial: u32 = 0;
    while engine.iter_all().count() != 1 {
        assert_trial_limit(&mut trial);
        engine.simulate(Default::default());
        let (water, _) = engine.iter_solvents().last().unwrap();
        if let Substance::Solution(_, _, solutes) = water {
            assert_eq!(solutes.len(), 1);
        }
    }
    trial = 0;

    println!("Dissolved all of vitae after {} trials", trial);
    let (water, solubility) = engine.iter_solvents().last().unwrap();

    let saturation = solubility.get_saturation_percent(&engine, water);
    println!("Final saturation is {}", saturation);

    // Add some saline.
    add_saline(&mut engine, saline_qty);
    engine.simulate(Default::default());

    let (water, _) = engine.iter_solvents().last().unwrap();
    if let Substance::Solution(_, _, solutes) = water {
        assert_eq!(solutes.len(), 2);
    }

    engine.simulate(Default::default());
    // Simulate until equilibrium is reached
    while !engine.is_in_equilibrium {
        assert_trial_limit(&mut trial);
        engine.simulate(Default::default());
    }
    println!("Reached equilibrium after {} trials", trial);

    // At this point there should be three substances:
    // - Solution of water + saline + vitae, saturated
    // - Free saline - approxomately 2/3 of added quantity
    // - Free vitae - approximately 1/3 of added quantity
    // Also total amount of reagents in the system should be the same as added.
    let mut vitae_total = Quantity::none();
    let mut saline_total = Quantity::none();
    engine.iter_all().for_each(|substance| match substance {
        Substance::Free(_, free_vitae) if free_vitae.essence_id == Essences::Vitae.into() => {
            assert!(Quantity::from(245) <= free_vitae.quantity);
            assert!(free_vitae.quantity <= Quantity::from(255));
            vitae_total += free_vitae.quantity;
        }
        Substance::Free(_, free_saline) if free_saline.essence_id == Essences::Saline.into() => {
            assert!(Quantity::from(490) <= free_saline.quantity);
            assert!(free_saline.quantity <= Quantity::from(510));
            saline_total += free_saline.quantity;
        }
        Substance::Solution(_, solution, solutes) => {
            let solubility = engine
                .get_essence(solution.essence_id)
                .unwrap()
                .solubility
                .unwrap();
            assert_eq!(
                solubility.get_saturation_percent(&engine, substance),
                1.0,
                "Solution should be saturated"
            );
            solutes.iter().for_each(|(&solute_essence, &solute_qty)| {
                if solute_essence == Essences::Vitae.into() {
                    vitae_total += solute_qty;
                } else if solute_essence == Essences::Saline.into() {
                    saline_total += solute_qty;
                } else {
                    panic!("Unexpected solute found in solution!")
                }
            })
        }
        _ => {
            panic!("Alchemy happened. Unexpected things found in the flask!")
        }
    });
    assert_eq!(vitae_total, vitae_qty);
    assert_eq!(saline_total, saline_qty);
}

#[test]
fn saturated_solution_precipitates_when_solvent_quantity_decreases() {
    let mut engine = setup();

    add_water(&mut engine, Quantity::default());
    add_saline(&mut engine, Quantity::from(1000));

    engine.simulate(Default::default());
    let mut trial = 0;
    // Let all of the salt dissolve
    while engine.iter_solutes().count() != 0 {
        assert_trial_limit(&mut trial);
        engine.simulate(Default::default());
    }

    // Add pyroflux and let the water start heating up and evaporating
    trial = 0;
    add_pyroflux(&mut engine);
    while !engine.iter_all().any(|p| p.get_form() == Forms::Gas.into()) {
        assert_trial_limit(&mut trial);
        engine.simulate(Default::default());
    }

    // Wait until all of the water has evaporated.
    trial = 0;
    while engine
        .iter_all()
        .any(|p| p.get_form() == Forms::Liquid.into())
    {
        assert_trial_limit(&mut trial);
        engine.simulate(Default::default());
    }

    // Check that we are at water boiling point
    assert!(engine.environment.temperature >= Temperature::from(100));

    // Assert that the following is true:
    // There are 3 free substances now since all of the water has evaporated:
    // - free salt (default amount)
    // - free steam (default amount)
    // - Some pyroflux (we don't care about)
    engine.iter_all().for_each(|substance| {
        if let Substance::Free(_, data) = substance {
            if data.essence_id == Essences::Aqua.into() {
                if data.form_id != Forms::Gas.into() {
                    panic!("Found non-steam water in the flask!")
                }
                assert_eq!(data.quantity, Quantity::default());
            }
            if data.essence_id == Essences::Saline.into() {
                if data.form_id != Forms::Crystalline.into() {
                    panic!("Found non-crystalline salt in the flask!")
                }
                assert_eq!(data.quantity, Quantity::default());
            }
        } else {
            panic!("There should be no solutions in here anymore!")
        }
    });
}
