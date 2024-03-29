use essentia_rs::Essence;

pub enum Essences {
    Pyroflux = 1
}

pub fn create_essences() -> Vec<Essence> {
    Vec::from([
        Essence::new_with_id(Essences::Pyroflux as u16, "Pyroflux")
    ])
}