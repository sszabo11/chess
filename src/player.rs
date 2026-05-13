use crate::peice::PieceData;

pub struct Player {
    pub pieces: Vec<PieceData>,
    pub moves: u32,
}

pub struct Coord(&'static str, u32);

impl Default for Player {
    fn default() -> Self {
        Self {
            pieces: Vec::new(),
            moves: 0,
        }
    }
}

impl Player {
    pub fn move_piece(coord: Coord) {}

    pub fn init_pieces(&mut self, rows: usize, cols: usize) {}
}
