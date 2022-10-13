use std::io;
use std::result::Result;

fn check_free_spaces(board: Vec<char>) -> bool {
    for i in board {
        if i == ' ' {
            return true;
        }
    }

    false
}

fn get_player_move(mut player: &char) -> Result<char, String> {
    let mut std_input = String::new();
    let mut tmp_char = ' ';

    match io::stdin().read_line(&mut std_input) {
        Ok(..) => tmp_char = std_input.chars().nth(0).unwrap(),
        Err(error) => println!("Error: {}", error),
    }

    if tmp_char != 'x' && tmp_char != 'o' {
        return Err(format!(
            "Invalid input. Expected 'x' or 'o', but got '{}'",
            tmp_char
        ));
    }

    Ok(tmp_char)
}

fn main() {
    let board = vec![' '; 9];
    let mut player = ' ';
    let mut computer = ' ';
    // let mut next_turn = 'x';
    // let mut winner = ' ';

    match get_player_move(&mut player) {
        Ok('x') => player = 'x',
        Ok('o') => player = 'o',
        Err(error) => println!("{}", error),
        _ => println!("Invalid input. Expected 'x' or 'o'."),
    }

    computer = if player == 'x' { 'o' } else { 'x' };
}
