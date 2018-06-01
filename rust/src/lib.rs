#![feature(entry_or_default)]
mod dfa;
mod div_dfa;
pub use dfa::Dfa;
pub use div_dfa::divisible_by;
