pub struct UltimateBoard {
    simple_boards: [SimpleBoard; 9],
    lock_board_index: Option<usize>,
    next_to_move: PlayerType,
}

struct SimpleBoard {
    squares: [Option<PlayerType>; 9],
    already_won: Option<PlayerType>,
    num_squares_placed: u8,
}

// allows us to initialze ultimate board with default values
impl Default for SimpleBoard {
    fn default() -> SimpleBoard {
        SimpleBoard {
            squares: [None; 9],
            already_won: None,
            num_squares_placed: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerType {
    X,
    O,
}

impl PlayerType {
    pub fn as_char(self) -> char {
        match self {
            PlayerType::X => 'X',
            PlayerType::O => 'O',
        }
    }

    pub fn next(self) -> PlayerType {
        match self {
            PlayerType::O => PlayerType::X,
            PlayerType::X => PlayerType::O,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MoveOutcome {
    pub player: PlayerType,
    pub board_index: usize,
    pub square_index: usize,

    pub won_board: bool,
    pub won_game: bool,
    pub tied_game: bool,
}

#[derive(Debug)]
pub enum MoveError {
    BoardOutOfBounds,
    SquareOutOfBounds,
    BoardLockViolated,
    SquareAlreadyTaken,
}

impl UltimateBoard {
    pub fn new() -> UltimateBoard {
        UltimateBoard {
            simple_boards: Default::default(),
            lock_board_index: None,
            next_to_move: PlayerType::X,
        }
    }

    // check if all of the sub boards are full
    fn is_full(&self) -> bool {
        for a in &self.simple_boards {
            if a.num_squares_placed != 9 {
                return false;
            }
        }

        true
    }

    pub fn get_locked_board(&self) -> Option<usize> {
        self.lock_board_index
    }

    pub fn get_next_to_move(&self) -> PlayerType {
        self.next_to_move
    }

    pub fn check_place_square(&self, board_index: usize, square_index: usize) -> Option<MoveError> {
        // make sure both indexes are in bounds, and the board
        // lock is being respected
        if square_index >= 9 || board_index >= 9 {
            return Some(MoveError::SquareOutOfBounds);
        }
        if board_index >= 9 {
            return Some(MoveError::BoardOutOfBounds);
        }
        if let Some(index) = self.lock_board_index {
            if board_index != index {
                return Some(MoveError::BoardLockViolated);
            }
        }
        if self.simple_boards[board_index].squares[square_index].is_some() {
            return Some(MoveError::SquareAlreadyTaken);
        }

        None
    }

    pub fn place_square(
        &mut self,
        board_index: usize,
        square_index: usize,
    ) -> Result<MoveOutcome, MoveError> {

        // make sure there is no errors with placing the square
        if let Some(e) = self.check_place_square(board_index, square_index) {
            return Err(e);
        }

        let player = self.next_to_move;
        let simple_board = &mut self.simple_boards[board_index];

        // the move succeeded, the next move should be locked to
        // this square, assuming it is not full
        if simple_board.num_squares_placed != 8 {
            self.lock_board_index = Some(square_index);
        } else {
            self.lock_board_index = None;
        }

        // place the square
        simple_board.squares[square_index] = Some(player);
        simple_board.num_squares_placed += 1;

        // all of the information needed is already set, we know
        // that there can not be a newly won board or a won game
        // because the board is already won, so its ok to just return
        if simple_board.already_won.is_some() {
            self.next_to_move = self.next_to_move.next();
            return Ok(MoveOutcome {
                player,
                board_index,
                square_index,
                won_board: false,
                won_game: false,
                tied_game: false,
            });
        }

        // we can now check if a board or game is won, and because
        // of the previous return that implies that it is new
        let won_board = is_win(|index| simple_board.squares[index]);

        // if it won, then for won_game to properly be set, we need
        // to tell it that this already won. It is also needed for next
        // move
        if won_board {
            simple_board.already_won = Some(self.next_to_move);
        }

        let won_game = is_win(|index| self.simple_boards[index].already_won);

        let tied_game = won_game == false && self.is_full();

        self.next_to_move = self.next_to_move.next();

        Ok(MoveOutcome {
            player,
            board_index,
            square_index,
            won_board,
            won_game,
            tied_game,
        })
    }

    // used for simplifing logic of to_string
    fn put_char(string: &mut String, square: Option<PlayerType>) {
        let c = match square {
            Some(player) => player.as_char(),
            None => ' ',
        };
        string.push(c);
    }

    fn put_small_board_row(&self, string: &mut String, board_index_start: usize) {
        for row_index in 0..3 {
            string.push('#');
            for board_index in board_index_start..=(board_index_start + 2) {
                let square_index = row_index * 3;
                let board = &self.simple_boards[board_index];
                Self::put_char(string, board.squares[square_index]);
                Self::put_char(string, board.squares[square_index + 1]);
                Self::put_char(string, board.squares[square_index + 2]);
                string.push('#');
            }
            string.push('\n');
        }

        // push the bottom chars
        for _ in 0..13 {
            string.push('#');
        }
        string.push('\n');
    }
}

impl ToString for UltimateBoard {
    fn to_string(&self) -> String {
        // 13 rows of 14 colums
        let mut buf = String::with_capacity(14 * 13);

        // add the starting row
        for _ in 0..13 {
            buf.push('#');
        }
        buf.push('\n');

        // add all of the single rows
        self.put_small_board_row(&mut buf, 0);
        self.put_small_board_row(&mut buf, 3);
        self.put_small_board_row(&mut buf, 6);

        buf
    }
}

// because the logic is the same for winning either the big or small board,
// this can be reused. It should take in the index (square or board) and return
// back whether there is a win or not
fn is_win(get_square: impl Fn(usize) -> Option<PlayerType>) -> bool {
    // check rows if they are equal
    nonempty_same(0, 1, 2, &get_square) ||
    nonempty_same(3, 4, 5, &get_square) ||
    nonempty_same(6, 7, 8, &get_square) ||

    // check columns if they are equal
    nonempty_same(0, 3, 6, &get_square) ||
    nonempty_same(1, 4, 7, &get_square) ||
    nonempty_same(2, 5, 8, &get_square) ||

    nonempty_same(0, 4, 8, &get_square) ||
    nonempty_same(2, 4, 6, &get_square)
}

fn nonempty_same(
    a: usize,
    b: usize,
    c: usize,
    get_square: impl Fn(usize) -> Option<PlayerType>,
) -> bool {
    let s = get_square(a);
    s.is_some() && s == get_square(b) && s == get_square(c)
}
