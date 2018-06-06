#![feature(entry_or_default)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(drain_filter)]
extern crate regex;

pub mod dfa;
mod div_dfa;
pub mod div_re;
mod gnfa;
mod simple_regex;
