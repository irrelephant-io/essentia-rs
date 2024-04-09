#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(hash_extract_if)]
#![feature(option_take_if)]

mod abstractions;
pub use abstractions::{
    physics, reaction, Environment, Essence, EssenceBuilder, EssenceId, Form, FormId, Substance,
    SubstanceBuilder, SubstanceId,
};

pub mod engine;

mod utils;
pub use utils::builder::Builder;
