use crate::framework::TwoPlayer;
use crate::shogi55::structs::piece::Piece;
use crate::shogi55::structs::possibility::Possibility;

#[derive(Debug, Clone)]
pub struct PieceInfo {
    player: TwoPlayer,
    piece: Piece,
    possibility: Possibility,
}

impl PieceInfo {
    pub fn new(player: &TwoPlayer, piece: &Piece) -> Self {
        Self {
            player: *player,
            piece: *piece,
            possibility: Possibility::new(),
        }
    }

    pub fn get_player(&self) -> &TwoPlayer {
        &self.player
    }

    pub fn get_piece(&self) -> &Piece {
        &self.piece
    }

    pub fn get_possibility(&self) -> &Possibility {
        &self.possibility
    }

    pub fn set_possibility(&mut self, possibility: &Possibility) {
        self.possibility = possibility.clone();
    }

    pub fn promote(&mut self) {
        self.piece = self.piece.promote()
    }
}
