#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
pub mod model;
pub mod utils;

pub trait Connection {}

pub trait Server {}
