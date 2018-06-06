extern crate clap;
extern crate div_regex;
use clap::{App, Arg};
use div_regex::div_re;

fn main() {
  let matches = App::new("div-re")
    .version("0.1")
    .author("Tej Chajed <tchajed@mit.edu>")
    .about("computes a regular expression to test divisibility by a specific modulus")
    .arg(Arg::with_name("modulus")
         .value_name("n")
         .help("modulus to test divisibility against")
         .required(true))
    .get_matches();
  let n = match matches.value_of("modulus").unwrap().parse::<u32>() {
    Ok(n) => n,
    Err(_) => panic!("n must be an integer"),
  };
  println!("{}", div_re::by_str(n))
}
