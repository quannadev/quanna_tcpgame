extern crate amethyst;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;

use amethyst::Result;
mod database;
mod managers;
mod models;
mod network;
mod schema;
use network::{Config, Networking};

fn main() -> Result<()> {
    let _run = Networking::new(Config::default());
    Ok(())
}
