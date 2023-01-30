use std::io;
static mut board:[[&str;3];3] = [["#","#","#"]
                ,["#","#","#"]
                ,["#","#","#"]];
static mut win:bool = false;
static mut turn:&str = "X";
fn check_vertical() -> bool{
    unsafe {
        for i in 0..board.len() {
            if((board[i][0] == turn && board[i][0] == board[i][1] && board[i][1] == board[i][2]) || (board[0][i] == turn && board[0][i] == board[1][i] && board[0][i] == board[2][i])) {
                return true;
            }
        }
        if((board[0][0] == turn && board[0][0] == board[1][1] && board[0][0] == board[2][2]) || (board[2][0] == turn && board[2][0] == board[1][1] && board[1][1] == board[0][2])) {
            return true;
        }
        if(turn == "X") {
            turn = "O";
        } else {
            turn = "X";
        }
    }
    false
}

fn insert(indexes:[usize;2]) -> bool{
    println!("{:?}", indexes);
    unsafe {
        if board[indexes[1]][indexes[0]] == "#" {
            board[indexes[1]][indexes[0]] = turn;
            return true;
        } 
    }
    false
}

fn get_user_input() {
    let mut position:[String; 2] = ["".to_string(),"".to_string()];
    let mut modifier:[usize;2] = [0, 0];
    loop {
        unsafe {
            println!("It's {}'s turn!", turn);
        }
        println!("Enter a x coordinate: ");
        io::stdin().read_line(&mut position[0]).expect("Failed to read line");
        println!("Enter a y coordinate: ");
        io::stdin().read_line(&mut position[1]).expect("Failed to read line");
        if (position[0].trim() == "1" || position[0].trim() == "2" || position[0].trim() == "3") && (position[1].trim() == "1" || position[1].trim() == "2" || position[1].trim() == "3") {
            modifier[0] = position[0].trim().parse().unwrap();
            modifier[1] = position[1].trim().parse().unwrap();
            modifier[0]-=1;
            modifier[1]-=1;
            if(insert(modifier) == true) {
                break;
            } else {
                println!("Spot already taken!");
            }
        } else {
            println!("Invalid Coordinates!");
            position = ["".to_string(),"".to_string()];
        }
    }
}
fn main() {
    unsafe {
        while win == false {
            get_user_input();
            win = check_vertical();
            for i in 0..board.len() {
                println!("Array {:?}", board[i]);
            }
        }
        println!("The winner is {}!", turn);
    }
}
