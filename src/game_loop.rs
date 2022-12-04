use super::{Opponent, UltimateTTTMove};
use crate::game::{MoveError, PlayerType, UltimateBoard};

fn input_usize_from_stdin() -> usize {
    let mut buffer = String::new();
    let input_stream = std::io::stdin();
    loop {
        input_stream.read_line(&mut buffer).unwrap();
        if let Ok(parsed_value) = buffer.trim().parse::<usize>() {
            return parsed_value;
        } else {
            println!("not a valid input, re-input");
        }
        buffer.clear();
    }
}

fn prompt_player_move(board: &UltimateBoard) -> UltimateTTTMove {
    let board_index;
    let square_index;
    // prompt based on the status of the board
    match board.get_locked_board() {
        Some(b) => {
            board_index = b;
            println!("locked into board {}", board_index + 1);
        }
        None => {
            println!("choose a board (1-9 inclusive): ");

            // make sure to not underflow when converting to 0-based index
            let raw_input_board_index = input_usize_from_stdin();
            board_index = if raw_input_board_index > 0 {
                raw_input_board_index - 1
            } else {
                usize::MAX
            }
        }
    };
    println!(
        "choose a square in board {} (1-9 inclusive): ",
        board_index + 1
    );

    let raw_input_square_index = input_usize_from_stdin();

    // make sure to not underflow when converting to 0-based index
    square_index = if raw_input_square_index > 0 {
        raw_input_square_index - 1
    } else {
        usize::MAX
    };

    UltimateTTTMove {
        board_index,
        square_index,
    }
}

// after this call the board will be updated, and the successful move
// will be returned
fn prompt_player_move_until_success(board: &UltimateBoard) -> UltimateTTTMove {
    // while there is not a valid move inputed
    loop {
        let ttt_move = prompt_player_move(&board);
        let UltimateTTTMove {
            board_index,
            square_index,
        } = ttt_move;

        // try the move and see if it works before sending it.
        // If it doesn't then print out the result and re-prompt
        use MoveError as ME;
        match board.validate_place_square(board_index, square_index) {
            Err(e) => match e {
                ME::SquareAlreadyTaken => {
                    println!("square is already taken");
                }
                ME::BoardLockViolated => {
                    println!("board lock violated");
                }
                ME::BoardOutOfBounds => {
                    println!("board not in the accepted range");
                }
                ME::SquareOutOfBounds => {
                    println!("square not in the accepted range");
                }
            },
            Ok(()) => return ttt_move,
        }
    }
}

pub fn start_game_loop(opponent: &mut impl Opponent, player: PlayerType) -> Result<(), ()> {
    // create the board
    let mut board = UltimateBoard::new();
    let mut message: Option<String> = None;

    // loop through until the game is over
    loop {
        // if player (me) is next to move, then take my input
        // and send it to the other player
        let move_outcome = if player == board.get_next_to_move() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("playing as {}", player.as_char());
            print!("{}", board.to_string());
            if let Some(message) = &message {
                println!("{}", message);
            }

            let ttt_move = prompt_player_move_until_success(&board);
            let move_outcome = board
                .place_square(ttt_move.board_index, ttt_move.square_index)
                .expect("invalid move not checked properly");

            opponent.send_move(ttt_move)?;
            move_outcome
        }
        // otherwise, the other player is next to move
        // read their input and update the board accordingly
        else {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("playing as {}", player.as_char());
            print!("{}", board.to_string());
            if let Some(message) = &message {
                println!("{}", message);
            }

            println!("waiting for opponent to make a move...");
            let ttt_move = opponent.receive_move()?;

            // the move should be valid, if sent correctly by the other player,
            // if it is not then close the stream

            let move_outcome = match board.place_square(ttt_move.board_index, ttt_move.square_index)
            {
                Ok(move_outcome) => move_outcome,
                Err(_) => return Err(()),
            };

            move_outcome
        };

        // reset the message to none if there is nothing new
        message = None;

        // process the move outcome
        if move_outcome.won_board {
            message = Some(format!(
                "player {} won board {}",
                move_outcome.player.as_char(),
                move_outcome.board_index
            ));
        }

        if move_outcome.tied_game {
            message = Some(format!("game is a tie"));
            break;
        } else if move_outcome.won_game {
            message = Some(format!("player {} won", move_outcome.player.as_char()));
            break;
        }
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("playing as {}", player.as_char());
    print!("{}", board.to_string());
    if let Some(message) = &message {
        println!("{}", message);
    }

    Ok(())
}
