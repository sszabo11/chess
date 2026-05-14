use crate::peice::Color;

#[derive(Debug)]
pub enum GameEvent {
    PlayerMove(u32, u32),
    SquareHighlight(u32, u32),
    PlayerResign(Color),
    None,
}
