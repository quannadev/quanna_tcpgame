extern crate amethyst;
#[macro_use]
extern crate log;
// #[macro_use]
extern crate serde;
// #[macro_use]
extern crate serde_json;

mod managers;
mod network;

use amethyst::Result as AmethystResult;
use network::{Config, Networking};

fn main() -> AmethystResult<()> {
    Networking::new(Config::default());
    Ok(())
}
