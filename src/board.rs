use std::cmp::{max, min};
use std::io;
use std::result::Result;

pub fn get_input_from_stdin() -> Result<String, io::Error> {
    let mut std_input = String::new();

    match io::stdin().read_line(&mut std_input) {
        Ok(..) => return Ok(std_input),
        Err(error) => return Err(error),
    }
}

pub fn check_free_spaces(board: Vec<char>) -> bool {
    for i in board {
        if i == ' ' {
            return true;
        }
    }

    false
}

pub fn put_char_at_position(board: &mut Vec<char>, pos: usize, char: char) -> Result<(), String> {
    if board[pos] != ' ' {
        return Err(format!(
            "Error placing '{}' at index '{}'. Found '{}'.",
            char, pos, board[pos]
        ));
    }

    board[pos] = char;

    Ok(())
}

pub fn get_player_character() -> Result<char, String> {
    let mut char_from_input = ' ';

    while char_from_input == ' ' {
        println!("Select between 'x' or 'o'.");

        match get_input_from_stdin() {
            Ok(input) => {
                char_from_input = input.chars().nth(0).unwrap();

                if char_from_input == 'x' || char_from_input == 'o' {
                    return Ok(char_from_input);
                } else {
                    char_from_input = ' ';
                }
            }
            Err(error) => return Err(format!("An unexpected error has occured.\n{:?}", error)),
        }
    }

    Err(format!("An unexpected error has occured.\n"))
}

pub fn player_turn(board: &mut Vec<char>, player_char: char) -> Result<(), String> {
    let mut pos = 0;

    while pos == 0 {
        println!("Your turn. Choose a position from 1 to 9.");

        match get_input_from_stdin() {
            Ok(input) => {
                pos = match input.trim().parse::<usize>() {
                    Ok(n) => {
                        if n < 1 || n > 9 {
                            0
                        } else {
                            n
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Expected a number between 1 and 9.");
                        0
                    }
                };
            }
            Err(error) => return Err(format!("An unexpected error has occured.\n'{:?}'", error)),
        }
    }

    put_char_at_position(board, pos - 1, player_char)
}

pub fn computer_turn(board: &mut Vec<char>, computer_char: char) -> Result<(), String> {
    let mut best_score = i32::MIN;
    let mut best_move = 0;

    for i in 0..9 {
        if board[i] == ' ' {
            board[i] = computer_char;
            let score = minimax(board, computer_char, false, 0);
            board[i] = ' ';

            if score > best_score {
                best_score = score;
                best_move = i;
            }
        }
    }

    put_char_at_position(board, best_move, computer_char)
}

pub fn check_if_someone_has_won(board: Vec<char>) -> Option<char> {
    let winning_pos = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for i in winning_pos {
        if board[i[0]] != ' ' && board[i[0]] == board[i[1]] && board[i[1]] == board[i[2]] {
            return Some(board[i[0]]);
        }
    }

    None
}

fn minimax(board: &mut Vec<char>, computer: char, is_maximizing: bool, depth: i32) -> i32 {
    let player = if computer == 'x' { 'o' } else { 'x' };

    if !check_free_spaces(board.to_vec()) {
        return 0;
    }

    match check_if_someone_has_won(board.to_vec()) {
        Some(c) => {
            if c == computer {
                return 10 - depth;
            } else if c == player {
                return -10 + depth;
            }
        }
        None => (),
    };

    let mut score = if is_maximizing { i32::MIN } else { i32::MAX };

    for i in 0..9 {
        if board[i] == ' ' {
            if is_maximizing {
                board[i] = computer;
                score = max(score, minimax(board, computer, !is_maximizing, depth + 1));
            } else {
                board[i] = player;
                score = min(score, minimax(board, computer, !is_maximizing, depth + 1));
            }
            board[i] = ' ';
        }
    }

    score
}
