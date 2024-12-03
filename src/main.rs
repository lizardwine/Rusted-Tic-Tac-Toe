mod board;

fn main() {

    let mut game = board::Board::new();
    game.print_board();    
    
    loop{
        let win = game.play();
        game.print_board();
        if win == 0{
            println!("Tie!");
            break;
        } else if win == 1{
            println!("X wins!");
            break;
        } else if win == 2{
            println!("O wins!");
            break;
        }
    }
    


}