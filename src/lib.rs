#![feature(fn_traits)]
#![feature(unboxed_closures)]

mod abstractions;
pub use abstractions::{
    Environment,
    Essence,
    EssenceBuilder,
    Form,
    Substance,
    SubstanceBuilder,
    SubstanceData,
    reaction,
    physics
};

pub mod engine;