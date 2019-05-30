use self::damage::*;
use self::motion::*;
use self::projectiles::*;
use self::scanner::*;
use crate::events::GameEvent;
use std::collections::HashMap;
use std::{ mpsc::Sender, Arc, RwLock };
use std::{ RwLockReadGuard, RwLockWriteGuard };

pub struct Gameloop {
    game_state: Arc<GameState>,
    systems: Vec<Box<System>>,
    cycle: u32,
    max_cycles: u32,
    num_combatants: usize,
}

#[derive(Debug)]
pub enum LoopTerminationReason {
    CycleCountExceeded,
}

pub trait System {
    fn apply(self: &Self, cycle: u32, game_state: &Arc<GameState>);
}

impl Gameloop {
    pub fn new(
        game_state: Arc<GameState>,
        max_cycles: u32,
        num_combatants: usize,
        logger: Option<Sender<GameEvent>>,
    ) -> Gameloop {
        Gameloop {
            game_state,
            systems: vec![
                Box::new(ScannerSystem::new(logger.clone())),
                Box::new(MotionSystem::new(logger.clone())),
                Box::new(ProjectileSystem::new(logger.clone())),
                Box::new(DamageSystem::new(logger.clone())),
            ],
            cycle: 0,
            max_cycles,
            num_combatants,
        }
    }

    pub fn start(&mut self) -> LoopTerminationReason {
        loop {
            self.systems
                .iter()
                .for_each( |s| s.apply(self.cycle, &self.game_state));

            self.cycle = self.cycle + 1;

            if self.cycle >= self.max_cycles {
                return LoopTerminationReason::CycleCountExceeded;
            }
        }
    }
}