use std::time::Duration;

use anyhow::Result;
use chess::{board::Board, peice::PieceKind};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    println!("---- Chess ----");

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
    let mut board = Board::new(8, 8, 4)
        .init_white(init_pos_white)
        .init_black(init_pos_black);

    //board.white.move(Coord("e", 5));

    let playing = true;
    board.term_board_row = 1;

    while playing {
        board.draw();
        let event = board.poll_input()?;
        println!("Eevntr: {:?}", event);

        board.tick(event);

        //board.wait_for_white().await?;
        //sleep(Duration::from_millis(100)).await;
    }
    //}
    //board.wait_for_black().await?;

    Ok(())
}
