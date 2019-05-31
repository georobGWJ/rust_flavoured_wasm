use super::*;
use::crate::events::log_event;
use crate::game::{ readlock, writelock };

pub struct DamageSystem {
    logger: Option<Sender<GameEvent>>,
}

impl System for DamageSystem {
    /// apply() mutates the damage component for each player in the game
    fn apply(&self, cycle: u32, game_state: &Arc<GameState>) {
        game_state.players.read().unwrap().iter().for_each( |p| {
            // get a writelock on the Component Hash
            writelock(&game_state.damage_components()
                // Get or create the damage_component
                .entry(p.to_string())
                // Modify the damage_component as needed
                .and_modify( |dc| self.advance(p, game_state, dc, cycle) ))
        });
    }
}
