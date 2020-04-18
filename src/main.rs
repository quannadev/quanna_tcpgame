extern crate amethyst;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
// #[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;

use amethyst::Result as AmethystResult;

mod database;
mod managers;
mod models;
mod network;
mod schema;

use network::{Config, Networking};

fn main() -> AmethystResult<()> {
    let config = Config::default();
    Networking::new(config);
    Ok(())
}
