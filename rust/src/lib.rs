#![feature(entry_or_default)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(drain_filter)]
extern crate regex;

mod dfa;
mod div_dfa;
mod div_re;
mod gnfa;
mod simple_regex;

pub use dfa::Dfa;
pub use div_re::by;
pub use gnfa::Gnfa;
