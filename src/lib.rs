pub struct Player {
    pub marker: String,
    pub turn: u32,
}

impl Player {
    pub fn new<'a>(new_marker: &str, new_turn: u32) -> Player {
        Player {marker: new_marker.to_string(), turn: new_turn}
    }
}

pub struct Board {
    pub tiles: [Box<[String]>; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [
                Box::new(["*".to_string(), "*".to_string(), "*".to_string()]),
                Box::new(["*".to_string(), "*".to_string(), "*".to_string()]),
                Box::new(["*".to_string(), "*".to_string(), "*".to_string()])
            ],
        }
    }

    pub fn show_board(&self) {
        println!("{:?} {:?} {:?}", self.tiles[0][0], self.tiles[0][1], self.tiles[0][2]);
        println!("{:?} {:?} {:?}", self.tiles[1][0], self.tiles[1][1], self.tiles[1][2]);
        println!("{:?} {:?} {:?}", self.tiles[2][0], self.tiles[2][1], self.tiles[2][2]);
    }

}

pub fn player_move(board: &mut Board, make_move: String, player: &Player) -> bool {
    let row_name = make_move[0..1].trim();
    let mut row_num: usize = match make_move[1..2].trim().parse() {
        Ok(usize) => usize,
        Err(_) => 0,
    };

    row_num -= 1;

    let marker = &player.marker;
    let turn: bool;

    if row_name == "a" {
        if board.tiles[0][row_num] != "*" {
            println!("Try again, not empty");
            turn = false;
        } else {
            board.tiles[0][row_num] = marker.to_string();
            turn = true;
        }
    }
    else if row_name == "b" {
        if board.tiles[1][row_num] != "*" {
            println!("Try again, not empty");
            turn = false;
        } else {
            board.tiles[1][row_num] = marker.to_string();
            turn = true;
        }
    }
    else {
        if board.tiles[2][row_num] != "*" {
            println!("Try again, not empty");
            turn = false;
        } else {
            board.tiles[2][row_num] = marker.to_string();
            turn = true;
        }
    }

    turn
}

pub fn check_for_winner(arr: &[Box<[String]>; 3], player1: &Player, player2: &Player) -> bool {
        let winner: bool;
        let marker1 = &player1.marker;
        let marker2 = &player2.marker;

        if arr[0][0] == marker1.to_string() && arr[0][1] == marker1.to_string() && arr[0][2] == marker1.to_string() || arr[0][0] == marker2.to_string() && arr[0][1] == marker2.to_string() && arr[0][2] == marker2.to_string() {
            winner = true;
        } 
        else if arr[1][0] == marker1.to_string() && arr[1][1] == marker1.to_string() && arr[1][2] == marker1.to_string() || arr[1][0] == marker2.to_string() && arr[1][1] == marker2.to_string() && arr[1][2] == marker2.to_string() {
            winner = true;
        }
        else if arr[2][0] == marker1.to_string() && arr[2][1] == marker1.to_string() && arr[2][2] == marker1.to_string() || arr[2][0] == marker2.to_string() && arr[2][1] == marker2.to_string() && arr[2][2] == marker2.to_string() {
            winner = true;
        }
        else if arr[0][0] == marker1.to_string() && arr[1][1] == marker1.to_string() && arr[2][2] == marker1.to_string() || arr[0][0] == marker2.to_string() && arr[1][1] == marker2.to_string() && arr[2][2] == marker2.to_string() {
            winner = true;
        }
        else if arr[0][2] == marker1.to_string() && arr[1][1] == marker1.to_string() && arr[2][0] == marker1.to_string() || arr[0][2] == marker2.to_string() && arr[1][1] == marker2.to_string() && arr[2][0] == marker2.to_string() {
            winner = true;
        }
        else if arr[0][0] == marker1.to_string() && arr[1][0] == marker1.to_string() && arr[2][0] == marker1.to_string() || arr[0][0] == marker2.to_string() && arr[1][0] == marker2.to_string() && arr[2][0] == marker2.to_string() {
            winner = true;
        }
        else if arr[0][1] == marker1.to_string() && arr[1][1] == marker1.to_string() && arr[2][1] == marker1.to_string() || arr[0][1] == marker2.to_string() && arr[1][1] == marker2.to_string() && arr[2][1] == marker2.to_string() {
            winner = true;
        }
        else if arr[0][2] == marker1.to_string() && arr[1][2] == marker1.to_string() && arr[2][2] == marker1.to_string() || arr[0][2] == marker2.to_string() && arr[1][2] == marker2.to_string() && arr[2][2] == marker2.to_string() {
            winner = true;
        }
        else {
            winner = false;
        }

        winner
    }