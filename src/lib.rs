#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(hash_extract_if)]
#![feature(option_take_if)]

mod abstractions;
pub use abstractions::{
    Environment,
    Essence,
    EssenceBuilder,
    Form,
    FormId,
    Substance,
    SubstanceBuilder,
    SubstanceId,
    EssenceId,
    reaction,
    physics
};

pub mod engine;

mod utils;
pub use utils::builder::Builder;