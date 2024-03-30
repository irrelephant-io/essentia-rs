use data::reaction::PyroflaxHeat;
use essentia_rs::{Environment, SubstanceBuilder};
use essentia_rs::engine::Essentia;
use essentia_rs::physics::{Quantity, TimeSpan};

use crate::data::essence::Essences;
use crate::data::form::Forms;
pub mod data;

fn setup() -> Essentia {
    let mut engine = Essentia::new(Environment::new());
    
    data::essence::create_essences()
        .into_iter()
        .for_each(|e| engine.register_essence(e));

    data::form::create_forms()
        .into_iter()
        .for_each(|f| engine.register_form(f));

    engine.register_reaction(PyroflaxHeat::default());

    engine
}

#[test]
fn simulate_empty_should_pass_time() {
    let mut engine = setup();
    
    let prev_time = engine.environment.time;
    engine.simulate(TimeSpan::from(10));

    assert_eq!(engine.substances.len(), 0);
    assert!(engine.environment.time > prev_time);
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

fn add_heatstone(engine: &mut Essentia) {
    engine.add_substance(
        SubstanceBuilder::new(&engine)
            .with_essence(Essences::Heatstone.into())
            .with_form(Forms::Salt.into())
            .with_quantity(Quantity::from(10))
            .build()
    );
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

    assert_eq!(engine.substances.len(), 1);
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