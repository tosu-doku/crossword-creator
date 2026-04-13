mod input;
mod board;
use std::collections::HashMap;

const START_HEIGHT: usize = 16;
const START_WIDTH: usize = 16;

type BoardType = Vec<Vec<char>>;
// type Location = (usize,usize);
// type LocationMap = HashMap<char,Vec<(usize,usize)>>;

fn main() {
    let mut board: BoardType = vec![vec![' '; START_WIDTH as usize]; START_HEIGHT as usize];

    let mut char_map = HashMap::new();
    
    let input: String = input::parse_input(); 
    // let toplace: &str = "kotek";
    board::place(&mut board, &mut char_map, &input);

    board::print_crossword(&board, (0,0), (START_WIDTH-1,START_HEIGHT-1));

    let (start, end) = board::find_borders(&board);
    println!("{},{};{},{}",start.0,start.1,end.0,end.1);
    println!("{:?}",char_map);

    board::print_crossword(&board, start, end);
}
