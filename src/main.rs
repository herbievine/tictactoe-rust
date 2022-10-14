mod board;

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn display_board(board: Vec<char>) -> () {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn display_winner(winner: char, player: char) -> () {
    if winner == player {
        println!("*** You Won! ***");
    } else if winner != player && winner != ' ' {
        println!("*** You lost! ***");
    } else {
        println!("*** You tied! ***");
    }
}

fn main() {
    let mut board = vec![' '; 9];
    let mut next_turn = 'x';
    let mut winner = ' ';

    clear();

    let player = match board::get_player_character() {
        Ok(c) => c,
        Err(error) => return println!("{}", error),
    };
    let computer = if player == 'x' { 'o' } else { 'x' };

    while winner == ' ' && board::check_free_spaces(board.clone()) {
        clear();
        display_board(board.clone());
        if next_turn == player {
            while next_turn == player {
                match board::player_turn(&mut board, player) {
                    Ok(_) => {
                        match board::check_if_someone_has_won(board.clone()) {
                            Some(c) => winner = c,
                            _ => (),
                        }
                        next_turn = computer;
                    }
                    Err(error) => println!("{}", error),
                }
            }
        } else if next_turn == computer {
            while next_turn == computer {
                match board::computer_turn(&mut board, computer) {
                    Ok(_) => {
                        match board::check_if_someone_has_won(board.clone()) {
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
