#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate hawk_tools;

pub mod model;
pub mod traits;

pub trait Connection {}

pub trait Server {}
