use rand::prelude::*;
use std::io;
use std::result::Result;

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_input_from_stdin() -> Result<String, io::Error> {
    let mut std_input = String::new();

    match io::stdin().read_line(&mut std_input) {
        Ok(..) => return Ok(std_input),
        Err(error) => return Err(error),
    }
}

fn check_free_spaces(board: Vec<char>) -> bool {
    for i in board {
        if i == ' ' {
            return true;
        }
    }

    false
}

fn display_board(board: Vec<char>) -> () {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---|---|---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---|---|---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn put_char_at_position(board: &mut Vec<char>, pos: usize, char: char) -> Result<(), String> {
    if board[pos] != ' ' {
        return Err(format!(
            "Error placing '{}' at index '{}'. Found '{}'",
            char, pos, board[pos]
        ));
    }

    board[pos] = char;

    Ok(())
}

fn get_player_character() -> Result<char, String> {
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

fn player_turn(board: &mut Vec<char>, player_char: char) -> Result<(), String> {
    let mut pos = 0;

    while pos == 0 {
        println!("Your turn. Choose a position from 1 to 9.");

        match get_input_from_stdin() {
            Ok(input) => {
                pos = input.trim().parse::<usize>().unwrap();

                if pos < 1 && pos > 9 {
                    pos = 0;
                }
            }
            Err(error) => return Err(format!("An unexpected error has occured.\n'{:?}'", error)),
        }
    }

    put_char_at_position(board, pos - 1, player_char)
}

fn computer_turn(board: &mut Vec<char>, computer_char: char) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let pos: usize = rng.gen_range(0..9);

    put_char_at_position(board, pos, computer_char)
}

fn check_if_someone_has_won(board: Vec<char>) -> Option<char> {
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

fn display_winner(winner: char, player: char) -> () {
    if winner == player {
        println!("****************");
        println!("*** You Won! ***");
        println!("****************");
    } else if winner != player && winner != ' ' {
        println!("*****************");
        println!("*** You lost! ***");
        println!("*****************");
    } else {
        println!("*****************");
        println!("*** You tied! ***");
        println!("*****************");
    }
}

fn main() {
    let mut board = vec![' '; 9];
    let mut player = ' ';
    let mut computer = ' ';
    let mut next_turn = 'x';
    let mut winner = ' ';

    clear();

    player = match get_player_character() {
        Ok(c) => c,
        Err(error) => return println!("{}", error),
    };
    computer = if player == 'x' { 'o' } else { 'x' };

    while winner == ' ' && check_free_spaces(board.clone()) {
        if next_turn == player {
            println!("Last turn was '{}' on index '{}'", player, computer);
            clear();
            display_board(board.clone());
            while next_turn == player {
                match player_turn(&mut board, player) {
                    Ok(_) => {
                        println!("Player went {}", player);
                        match check_if_someone_has_won(board.clone()) {
                            Some(c) => winner = c,
                            _ => (),
                        }
                        next_turn = computer;
                    }
                    Err(error) => println!("{}", error),
                }
            }
        } else if next_turn == computer {
            println!("Last turn was '{}' on index '{}'", player, computer);
            clear();
            display_board(board.clone());
            while next_turn == computer {
                match computer_turn(&mut board, computer) {
                    Ok(_) => {
                        println!("Computer went {}", computer);
                        match check_if_someone_has_won(board.clone()) {
                            Some(c) => winner = c,
                            _ => (),
                        }
                        next_turn = player;
                    }
                    Err(error) => println!("{}", error),
                }
            }
        }
    }

    clear();
    display_winner(winner, player);
    display_board(board);
}
