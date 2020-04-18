extern crate amethyst;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
use amethyst::Result;
mod managers;
mod network;
use network::{Config, Networking};

fn main() -> Result<()> {
    let _run = Networking::new(Config::default());
    Ok(())
}
