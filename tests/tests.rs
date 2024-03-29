use data::reaction::PyroflaxHeat;
use essentia_rs::{Environment, Substance, Quantity, SubstanceData, Time};
use essentia_rs::engine::Essentia;

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
    engine.simulate(&Time::from(10));

    assert_eq!(engine.substances.len(), 0);
    assert!(engine.environment.time > prev_time);       
}

#[test]
fn simulate_simple_exotherm() {
    let mut engine = setup();

    engine.add_substance(
        Substance::Normal(
            SubstanceData {
                essence_id: Essences::Pyroflux as u16,
                form_id: Forms::Salt as u16,
                quantity: Quantity::default()
            }
        )
    );

    // Since pyroflux is emitting heat, we expect the temperature of the ENV to rise.
    let old_temp = engine.environment.temperature;
    engine.simulate(&Time::from(1));

    assert_eq!(engine.substances.len(), 1);
    assert!(old_temp < engine.environment.temperature);
}