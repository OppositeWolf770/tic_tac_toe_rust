fn main() {

    let spot = 'X';

    let mut board = [
        [spot, spot, spot],
        [spot, spot, spot],
        [spot, spot, spot],
    ];

    let mut count:u32 = 0;

    while 1 != 2 {
        take_turn((count % 2 + 1), board);
        count = count + 1;
    }

}

// Allows the user to take their turn
fn take_turn(player:u32, board:[[char; 3]; 3]) {
    println!("Player {player}");
    display_board(board);
    pick_space(player, board);
}

// Displays the board to the user
fn display_board(board:[[char; 3]; 3]) {
    for row in board {
        for spot in row {
            print!("{spot}");
        }
        println!();
    }

}

// Allows the user to pick the spot they want to play
fn pick_space(player:u32, board:[[char;3];3]) {

}

fn is_valid(row:u8, col:u8, board:[[char;3];3]) -> bool {


    return true;
}

fn check_winner(board:[[char;3];3]) -> u8 {
    return 1;
}
