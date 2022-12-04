use super::{Opponent, UltimateTTTMove};
use crate::game::UltimateBoard;

pub struct AiOpponent {
    board: UltimateBoard,
}

impl AiOpponent {
    pub fn new(board: UltimateBoard) -> AiOpponent {
        AiOpponent { board }
    }
}

// allow calls for send/receive move on Box<dyn Opponent>
impl<O: Opponent + ?Sized> Opponent for Box<O> {
    fn send_move(&mut self, ttt_move: UltimateTTTMove) -> Result<(), ()> {
        self.as_mut().send_move(ttt_move)
    }
    
    fn receive_move(&mut self) -> Result<UltimateTTTMove, ()> {
        self.as_mut().receive_move()
    }
}

impl Opponent for AiOpponent {
    fn send_move(&mut self, ttt_move: UltimateTTTMove) -> Result<(), ()> {
        self.board
            .place_square(ttt_move.board_index, ttt_move.square_index)
            .expect("move is invalid");

        Ok(())
    }

    fn receive_move(&mut self) -> Result<UltimateTTTMove, ()> {
        for i in 0..9 {
            for j in 0..9 {
                if let Ok(()) = self.board.validate_place_square(i, j) {
                    self.board.place_square(i, j).unwrap();
                    return Ok(UltimateTTTMove {
                        board_index: i,
                        square_index: j,
                    });
                }
            }
        }
    
        panic!("there are no valid moves, but it is asking for a move");
    }
}
