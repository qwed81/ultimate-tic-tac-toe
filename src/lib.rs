pub mod game;
pub mod game_loop;
pub mod network_opponent;
pub mod ai_opponent;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UltimateTTTMove {
    square_index: usize,
    board_index: usize,
}

pub trait Opponent {
    // sends a move to the opponent
    fn send_move(&mut self, ttt_move: UltimateTTTMove) -> Result<(), ()>;

    // receives a move from the opponent, or blocks until one
    // is received
    fn receive_move(&mut self) -> Result<UltimateTTTMove, ()>;
}
