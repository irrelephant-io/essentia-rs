use essentia_rs::engine::{Essentia, EssentiaBuilder};

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

    builder.build()
}

#[test]
fn test_form_transitions() {
    let mut _engine = setup();
}