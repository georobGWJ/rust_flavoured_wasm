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

impl DamageSystem {

    pub fn new(logger: Option<Sender<GameEvent>>) -> DamageSystem {
        DamageSystem { logger }
    }

    pub fn advance(
        &self,
        player: &str,
        game_state: &Arc<GameState>,
        dc: &mut DamageComponent,
        cycle: u32,
        ) {
        self.apply_collision_damage(player, game_state, dc, cycle);
        self.apply_projectile_damage(player, game_state, dc, cycle);
        self.check_for_death(player, dc, cycle);
    }

    fn check_for_death(&self, player: &str, dc: &mut DamageComponent, cycle: u32) {
        if dc.damage >= DAMAGE_MAX && !dc.dead() {
            dc.damage = DAMAGE_MAX;
            dc.status = DamageStatus::Dead;

            log_event(
                &self.logger,
                GameEvent::Death {
                    cycle,
                    victim: player.to_string(),
                },
            );
        }
    }

    fn apply_collision_damage(
        &self,
        player: &str,
        game_state: &Arc<GameState>,
        dc: &mut DamageComponent,
        cycle: u32,
        ) {
        let mcs = readlock(&game_state.motion_components);
        let mc_opt = mcs.get(player);

        match mc_opt {
            Some(mc) => match mc.collision {
                Some(CollisionType::Player(ref p)) => {
                    dc.add_damage(DAMAGE_COLLISION);
                    self.log_damage(
                        cycle, 
                        DAMAGE_COLLISION,
                        DamageKind::Collision(CollisionType::Player(p.to_string())),
                    );
                },
                None => {}
            },
            None => {}
        }
    }

    fn apply_projectile_damage(
        &self,
        player: &str,
        game_state: &Arc<GameState>,
        dc: &mut DamageComponent,
        cycle: u32,
        ) {
        let pcs = game_state.projectile_components.read().unwrap();
        let pc_opt = pcs.get(player);

        match pc_opt {
            Some(pc) => {
                for x in 0..1 {
                    if pc.projectiles[x].active_hits.contains_key(player) {
                        let dmg: u32 = pc.projectiles[x].active_hits[player];
                        println!("Doing explosion damage {} to player {}", dmg, player);
                        dc.add_damage(dmg);
                        self.log_damage(cycle, dmg, DamageKind::Projectile, player);
                    }
                }
            },
            None => {}
        }
    }

    fn log_damage(&self, cycle: u32, amount: u32, kind: DamageKind, victim: &str) {
        log_event(
            &self.logger,
            GameEvent::Damage { 
                cycle,
                amount, 
                kind, 
                victim: victim.to_string(),
            },
        );
    }
}