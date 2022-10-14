pub fn logo() -> () {
    println!("  __  .__                   __                            __");
    println!("_/  |_|__| ____           _/  |______    ____           _/  |_  ____   ____");
    println!(
        "\\   __\\  |/ ___\\   ______ \\   __\\__  \\ _/ ___\\   ______ \\   __\\/  _ \\_/ __ \\ "
    );
    println!(" |  | |  \\  \\___  /_____/  |  |  / __ \\\\  \\___  /_____/  |  | (  <_> )  ___/ ");
    println!(" |__| |__|\\_____>          |__| (______/\\_____>          |__|  \\____/ \\_____>");
}

pub fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn display_board(board: Vec<char>) -> () {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

pub fn display_winner(winner: char, player: char) -> () {
    if winner == player {
        println!("*** You Won! ***");
    } else if winner != player && winner != ' ' {
        println!("*** You lost! ***");
    } else {
        println!("*** You tied! ***");
    }
}
