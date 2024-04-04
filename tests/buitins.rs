use data::reactions::CryodustChill;
use data::{essence::Essences, reactions::PyroflaxHeat};
use data::form::Forms;
use essentia_rs::physics::Power;
use essentia_rs::Substance;
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
            .is_normal()
            .with_essence(Essences::Pyroflux.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::default())
            .build()
    );
}

fn add_cryodust(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Cryodust.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::from(10))
            .build()
    );
}

fn add_water(engine: &mut Essentia, quantity: Quantity) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Aqua.into())
            .with_quantity(quantity)
            .with_form(Forms::Liquid.into())
            .build()
    );
}

fn add_vitae(engine: &mut Essentia, quantity: Quantity) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Vitae.into())
            .with_quantity(quantity)
            .with_form(Forms::Crystalline.into())
            .build()
    );
}

fn get_of_form(engine: &Essentia, form: Forms) -> impl Iterator<Item = &Substance> {
    engine
        .iter_all()
        .filter(move |s| {
            s.is_form(form.into())
        })
}

fn get_of_essense(engine: &Essentia, essence: Essences) -> impl Iterator<Item = &Substance> {
    engine
        .iter_all()
        .filter(move |s| {
            s.is_essence(essence.into())
        })
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
    add_water(&mut engine, Quantity::from(10));
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
        let liquid_count = get_of_form(&engine, Forms::Liquid.into()).count();
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
        let liquid_count = get_of_form(&engine, Forms::Liquid.into()).count();
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

#[test]
fn test_solution_in_water() {
    let mut engine = setup();
    let starting_vitae = Quantity::from(50);
    add_water(&mut engine, Quantity::from(20));
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
                    let vitae_solute = solutes.last().unwrap();
                    assert_eq!(vitae_solute.essence_id, Essences::Vitae.into());
                    assert!(vitae_solute.quantity < starting_vitae);
                    Some(vitae_solute.quantity)
                }
            }
        })
        .sum::<Quantity>();
    assert_eq!(total_vitae, starting_vitae);

    let mut trial: u32 = 0;
    while get_of_essense(&engine, Essences::Vitae.into()).count() != 1 {
        assert_trial_limit(&mut trial);
        engine.simulate(TimeSpan::default());
    }
}