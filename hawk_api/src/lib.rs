#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate hawk_tools;

pub mod config_center;
pub mod model;
pub mod traits;

pub const CONNECTION_CHAR: char = '@';
