#![feature(entry_or_default)]
mod dfa;
mod div_dfa;
mod regex;
mod gnfa;

pub use dfa::Dfa;
pub use div_dfa::divisible_by;
pub use gnfa::Gnfa;
