#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}
impl GamePiece {
    // Create a regular uncrowned piece
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }

    // Create a crowned piece
    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece {
            color: p.color,
            crowned: true,
        }
    }
}