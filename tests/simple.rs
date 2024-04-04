use data::reactions::{PyroflaxHeat, CryodustChill};

use essentia_rs::SubstanceBuilder;
use essentia_rs::engine::{Essentia, EssentiaBuilder};
use essentia_rs::physics::{Quantity, TimeSpan};

use crate::data::essence::Essences;
use crate::data::form::Forms;

pub mod data;

fn setup() -> Essentia {
    // Create engine without built-in reactions
    let mut builder = EssentiaBuilder::new();
    
    data::essence::create_essences()
        .into_iter()
        .for_each(|e| builder.register_essence(e));

    data::form::create_forms()
        .into_iter()
        .for_each(|f| builder.register_form(f));

    builder.register_reaction(Box::new(PyroflaxHeat::default()));
    builder.register_reaction(Box::new(CryodustChill::default()));

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

fn add_inertia(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Inertia.into())
            .with_form(Forms::Gas.into())
            .with_quantity(Quantity::default())
            .build()
    );
}

fn add_heatstone(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .is_normal()
            .with_essence(Essences::Heatstone.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::from(10))
            .build()
    );
}

#[test]
fn simulate_empty_should_pass_time() {
    let mut engine = setup();
    
    let prev_time = engine.environment.time;
    engine.simulate(TimeSpan::from(10));

    assert_eq!(engine.iter_all().count(), 0);
    assert!(engine.environment.time > prev_time);
}

#[test]
fn simulate_simple_exotherm() {
    let mut engine = setup();
    add_pyroflux(&mut engine);

    // Since pyroflux is emitting heat, we expect the temperature of the ENV to rise.
    let temp_sample_pre = engine.environment.temperature;
    engine.simulate(TimeSpan::from(1));
    let temp_sample_1 = engine.environment.temperature;
    engine.simulate(TimeSpan::from(2));
    let temp_sample_2 = engine.environment.temperature;

    assert_eq!(engine.iter_all().count(), 1);
    assert!(temp_sample_pre < engine.environment.temperature);
    assert!(
        temp_sample_2 - temp_sample_1 > temp_sample_1 - temp_sample_pre,
        "Change in temp should depend on time!"
    );
}

#[test]
fn exotherm_heats_up_less_in_presense_of_larger_heat_cap() {
    let mut just_pyro_engine = setup();
    add_pyroflux(&mut just_pyro_engine);

    let mut pyro_heatstone_engine = setup();
    add_pyroflux(&mut pyro_heatstone_engine);
    add_heatstone(&mut pyro_heatstone_engine);

    just_pyro_engine.simulate(TimeSpan::from(1000));
    pyro_heatstone_engine.simulate(TimeSpan::from(1000));

    println!(
        "just_pyro (T): {:?}, pyro_with_heatstone (T): {:?}",
        just_pyro_engine.environment.temperature,
        pyro_heatstone_engine.environment.temperature
    );
    assert!(just_pyro_engine.environment.temperature > pyro_heatstone_engine.environment.temperature)
}

#[test]
fn inertia_doesnt_do_anything() {
    let mut just_pyro_engine = setup();
    add_pyroflux(&mut just_pyro_engine);

    let mut pyro_inertia_engine = setup();
    add_pyroflux(&mut pyro_inertia_engine);
    add_inertia(&mut pyro_inertia_engine);

    just_pyro_engine.simulate(TimeSpan::from(1000));
    pyro_inertia_engine.simulate(TimeSpan::from(1000));

    println!(
        "just_pyro: {:?}, pyro_with_inertia:{:?}",
        just_pyro_engine.environment,
        pyro_inertia_engine.environment
    );
    assert_eq!(just_pyro_engine.environment.temperature, pyro_inertia_engine.environment.temperature);
}

fn get_quantity_of(engine: &Essentia, essence: Essences) -> Quantity {
    engine
        .iter_all()
        .filter_map(|s| {
            if s.get_essence() == essence.into() {
                Some(s.get_quantity())
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn cryo_is_consumed_over_time() {
    let mut engine = setup();
    add_cryodust(&mut engine);

    let temp_sample_pre = engine.environment.temperature;
    let cryo_pre = get_quantity_of(&engine, Essences::Cryodust);
    println!("pre_temp: {:?}, pre_qty: {:?}", temp_sample_pre, cryo_pre);
    engine.simulate(TimeSpan::from(100)); 
    let cryo_1 = get_quantity_of(&engine, Essences::Cryodust);
    let temp_sample_1 = engine.environment.temperature;
    println!("pre_temp: {:?}, pre_qty: {:?}", temp_sample_1, cryo_1);
    // By this time we expect no change in the environment since all of the cryo was consumed by the reaction
    engine.simulate(TimeSpan::from(100));
    let cryo_2 = get_quantity_of(&engine, Essences::Cryodust);
    let temp_sample_2 = engine.environment.temperature;
    println!("pre_temp: {:?}, pre_qty: {:?}", temp_sample_2, cryo_2);

    assert!(temp_sample_pre > temp_sample_1);
    assert!(cryo_pre > cryo_1);
    assert_eq!(temp_sample_2, temp_sample_1);
    assert_eq!(cryo_2, cryo_1);
    assert_eq!(engine.iter_all().count(), 0);
}