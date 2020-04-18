// use crate::managers::connections::ConnectionManager;
use amethyst::{GameData, SimpleState, StateData};

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Init Game States success")
    }
}
