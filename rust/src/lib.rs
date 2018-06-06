#![feature(entry_or_default)]
#![feature(box_syntax)]
#![feature(box_patterns)]
extern crate regex;

mod dfa;
mod div_dfa;
mod simple_regex;
mod gnfa;

pub use dfa::Dfa;
pub use div_dfa::divisible_by;
pub use gnfa::Gnfa;
