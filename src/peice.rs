#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}
#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
    Pawn,
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
