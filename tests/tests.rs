use data::reaction::PyroflaxHeat;
use essentia_rs::{Environment, Substance, SubstanceData};
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
        Substance::Normal(
            SubstanceData {
                essence_id: Essences::Pyroflux as u16,
                form_id: Forms::Salt as u16,
                quantity: Quantity::default()
            }
        )
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