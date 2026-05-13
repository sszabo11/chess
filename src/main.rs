use anyhow::Result;
use chess::{board::Board, peice::PieceKind};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Chess");

    let init_pos_white = vec![
        [
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
        ],
        [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ],
    ];

    let init_pos_black = vec![
        [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ],
        [
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
            PieceKind::Pawn,
        ],
    ];
    let mut board = Board::new(6, 8)
        .init_white(init_pos_white)
        .init_black(init_pos_black);

    //board.white.move(Coord("e", 5));

    let playing = true;

    //while playing {
    //    //board.draw(8);

    //    board.wait_for_black().await?;
    //}
    board.wait_for_black().await?;
    //board.wait_for_white().await?;

    Ok(())
}
