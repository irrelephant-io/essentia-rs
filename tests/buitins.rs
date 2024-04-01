use data::reactions::CryodustChill;
use data::{essence::Essences, reactions::PyroflaxHeat};
use data::form::Forms;
use essentia_rs::physics::Power;
use essentia_rs::{engine::{Essentia, EssentiaBuilder}, physics::{TimeSpan, Quantity, Temperature, Rate}, SubstanceBuilder};

pub mod data;

fn setup() -> Essentia {
    // Create engine WITH built-in reactions
    let mut builder = EssentiaBuilder::default();
    
    data::essence::create_essences()
        .into_iter()
        .for_each(|e| builder.register_essence(e));

    data::form::create_forms()
        .into_iter()
        .for_each(|f| builder.register_form(f));

    builder.register_reaction(Box::new(PyroflaxHeat::from(42)));
    builder.register_reaction(Box::new(CryodustChill::new(Power::from(40), Rate::from(0))));

    builder.build()
}

fn add_pyroflux(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .with_essence(Essences::Pyroflux.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::default())
            .build()
    );
}

fn add_cryodust(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .with_essence(Essences::Cryodust.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::from(10))
            .build()
    );
}

fn add_water(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .with_essence(Essences::Aqua.into())
            .with_quantity(Quantity::from(10))
            .with_form(Forms::Liquid.into())
            .build()
    );
}

const TRIAL_LIMIT: u32 = 1000;
const WATER_BOIL_TEMP: Temperature = Temperature { degrees: 100 };
const WATER_CRYSTALLIZATION_TEMP: Temperature = Temperature { degrees: 0 };

fn assert_trial_limit(trial: &mut u32) {
    *trial += 1;
    if *trial >= TRIAL_LIMIT {
        panic!("Trial limit reached!");
    }
}

#[test]
fn test_water_evaporation_transitions() {
    let mut engine = setup();
    add_water(&mut engine);
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
        let liquid_count = engine.get_of_form(Forms::Liquid.into()).into_iter().count();
        if liquid_count == 0 {
            // We have succesfully evaporated all of the water. Now there is only steam!
            break;
        }
        assert_eq!(
            engine.environment.temperature,
            WATER_BOIL_TEMP,
            "While form transition is happening, all energy goes to transition."
        )
    }

    assert_eq!(engine.get_form(Forms::Gas.into()).into_iter().count(), 1);
}


#[test]
fn test_water_crystalization_transitions() {
    let mut engine = setup();
    add_water(&mut engine);
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
        let liquid_count = engine.get_of_form(Forms::Liquid.into()).into_iter().count();
        if liquid_count == 0 {
            // We have succesfully evaporated all of the water. Now there is only steam!
            break;
        }
        assert_eq!(
            engine.environment.temperature,
            WATER_CRYSTALLIZATION_TEMP,
            "While form transition is happening, all energy goes to transition."
        )
    }

    assert_eq!(engine.get_form(Forms::Crystalline.into()).into_iter().count(), 1);
}