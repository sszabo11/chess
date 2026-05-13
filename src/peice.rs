trait Peice {
    fn goto(&self, dest: (u32, u32));
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

//#[derive(Debug, Clone, Copy)]
//pub enum Piece {
//    //Rook(u32, u32),
//    //Bishop(u32, u32),
//    //Knight(u32, u32),
//    //Queen(u32, u32),
//    //King(u32, u32),
//    //Pawn(u32, u32),
//    Piece(PieceData),
//}
#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
    Pawn,
    //Rook(u32, u32),
    //Bishop(u32, u32),
    //Knight(u32, u32),
    //Queen(u32, u32),
    //King(u32, u32),
    //Pawn(u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub struct PieceData {
    pub kind: PieceKind,
    pub row: u32,
    pub col: u32,
}

pub fn print_piece(piece: &PieceKind, color: Color) -> String {
    let s = if color == Color::Black {
        match piece {
            PieceKind::Rook => "♜",
            PieceKind::Knight => "♞",
            PieceKind::Pawn => "♟",
            PieceKind::King => "♚",
            PieceKind::Queen => "♛",
            PieceKind::Bishop => "♝",
            _ => "?",
        }
    } else {
        match piece {
            PieceKind::Rook => "♖",
            PieceKind::Knight => "♘",
            PieceKind::Pawn => "♙",
            PieceKind::King => "♔",
            PieceKind::Queen => "♕",
            PieceKind::Bishop => "♗",
            _ => "?",
        }
    };

    s.to_string()
}

// Trait Peice defines any Peice
// Each peice must:
// move to square
// avaiabe moves, based on rellative location. eg. all current row, L, etc
// be captured/removed
//
