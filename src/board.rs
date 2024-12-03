use std::io::Write;

pub struct Board{
    board: [[char; 3]; 3],
    available: [bool; 9],
    xturn: bool,
}
impl Board{
    pub fn new() -> Board{
        Board{
            board: [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']],
            available: [true; 9],
            xturn: true,
        }
    }
    pub fn print_board(&self){
        println!(" {} | {} | {}", self.board[0][0], self.board[0][1], self.board[0][2]);
        println!("---+---+---");
        println!(" {} | {} | {}", self.board[1][0], self.board[1][1], self.board[1][2]);
        println!("---+---+---");
        println!(" {} | {} | {}", self.board[2][0], self.board[2][1], self.board[2][2]);
    }

    pub fn is_win(&self) -> usize{
        // check if X or O has won or tie
        // 0 tie, 1 X wins, 2 O wins
        let wins = [
            [[0, 0], [0, 1], [0, 2]], [[0, 0], [1, 0], [2, 0]],
            [[0, 0], [1, 1], [2, 2]], [[0, 2], [1, 1], [2, 0]],
            [[0, 1], [1, 1], [2, 1]], [[0, 2], [1, 2], [2, 2]],
            [[1, 0], [1, 1], [1, 2]], [[2, 0], [2, 1], [2, 2]]
        ];
    
        for &player in &['X', 'O'] {
            for win in &wins {
                if win.iter().all(|&[y, x]| self.board[y][x] == player) {
                    return if player == 'X' { 1 } else { 2 };
                }
            }
        }
        0
    }

    pub fn play(&mut self) -> usize{
        print!("{}'s Turn: ", if self.xturn { "X" } else { "O" });
        std::io::stdout().flush().unwrap();
        //read input
        let mut input;
        let mut input_int: usize;
        loop{
            input = String::new();

            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<usize>(){
                Ok(num) => input_int = num,
                Err(_) => {
                    println!("Please type a number!");
                    print!("{}'s Turn: ", if self.xturn { "X" } else { "O" });
                    std::io::stdout().flush().unwrap();
                    continue;
                }
            };

            if !(input_int > 0 && input_int < 10){
                println!("Please type a number between 1 and 9");
                print!("{}'s Turn: ", if self.xturn { "X" } else { "O" });
                std::io::stdout().flush().unwrap();
                continue;
            }
            if !self.available[input_int-1]{
                println!("This square is already taken");
                print!("{}'s Turn: ", if self.xturn { "X" } else { "O" });
                std::io::stdout().flush().unwrap();
                continue;
            }
            break;
        }
        
        self.board[(input_int-1)/3][(input_int-1)%3] = if self.xturn { 'X' } else { 'O' };
        self.xturn = !self.xturn;
        self.available[input_int-1] = false;
        let win = self.is_win();
        if win != 0 {
            return win;
        } else if win == 0 && !self.available.iter().any(|&x| x == true){
            return win;
        }
        return 3;
    }
}