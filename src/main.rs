mod board;
mod display;

fn main() {
    let mut board = vec![' '; 9];
    let mut next_turn = 'x';
    let mut winner = ' ';

    display::clear();
    display::logo();

    let player = match board::get_player_character() {
        Ok(c) => c,
        Err(error) => return println!("{}", error),
    };
    let computer = if player == 'x' { 'o' } else { 'x' };

    while winner == ' ' && board::check_free_spaces(board.clone()) {
        display::clear();
        display::logo();
        display::display_board(board.clone());
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

    display::clear();
    display::logo();
    display::display_winner(winner, player);
    display::display_board(board);
}
