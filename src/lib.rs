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
    SubstanceData,
    EssenceId,
    reaction,
    physics
};

pub mod engine;

mod utils;
pub use utils::builder::Builder;
pub use utils::either::Either;