use std::num::ParseIntError;

type Board = [[char;3];3];
type IntResult = Result<u8, ParseIntError>;

fn main() {
    let spot = ' ';

    let mut board: Board = [
        [spot, spot, spot],
        [spot, spot, spot],
        [spot, spot, spot],
    ];

    let mut count:u32 = 0;

    pick_space(1, board);
}

// Allows the user to take their turn
fn take_turn(player:u32, board:Board) {
    println!("Player {player}");
    display_board(board);
    pick_space(player, board);
}

// Displays the board to the user
fn display_board(board:Board) {
    for row in board {
        for spot in row {
            print!("{spot}");
        }
        println!();
    }
}

// Allows the user to pick the spot they want to play
fn pick_space(player:u32, board:Board) {

    loop {
        // Grabs the row and column desired from the user
        let row = get_input("Choose a row:") - 1;
        let col = get_input("Choose a column:") - 1;

        if is_valid(row, col, board) {
            break;
        }
    }
}


fn check_winner(board:Board) -> u8 {
    return 1;
}

// Gets the input from the user
fn get_input(prompt:&str) -> u8 {
    let mut input = String::new(); // Input buffer
    
    // Loops until a valid value is entered
    loop {
        println!("{prompt}");

        // Gets the user input
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read user input.");
    
        // Stores the result in a variable to be tested
        let value: IntResult = input
            .trim()
            .parse();

            // Tests if the input is a valid number
            match value {
                Ok(num) => {
                    if num > 3 || num < 1 { // Input must be between 1 and 3, inclusively
                        println!("You did not enter a correct value!");
                        input.clear(); // Clears the input buffer
                    } else {
                        return num;
                    }
                }
                Err(_) => {
                    println!("You did not enter a correct value!");
                    input.clear(); // Clears the input buffer
                }
            }
    }

}


fn is_valid(row:u8, col:u8, board:Board) -> bool {
    let row = row as usize;
    let col = col as usize;
    if board[row][col] != ' ' {
        println!("Invalid move! Choice must be in an empty spot.");
        return false;
    }

    return true;
}
