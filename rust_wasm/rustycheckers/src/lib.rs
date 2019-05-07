#[macro_use]
extern crate lazy_static;

// lazy_static allows creation of a globally available instance of
// the GameEngine struct. 
use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use mut_static::MutStatic;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> =
        { MutStatic::from(GameEngine::new()) };
}

mod board;
mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
