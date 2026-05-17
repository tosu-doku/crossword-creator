mod input;
mod board;
mod config;
use config::*;
mod gui;

fn main() {
    let mut board: BoardType = vec![vec![' '; START_WIDTH as usize]; START_HEIGHT as usize];

    let mut char_map: LocationMap;
    let mut word_start_arrows: LocationMapDirection = HashMap::new();
    
    let mut list_of_words: Vec<&str> = vec!["pies","konik","chomik","ptak","kot"];
    (board,char_map) = board::return_crossword(&mut list_of_words,&mut word_start_arrows);

    board::print_crossword(&board, (0,0), (START_WIDTH-1,START_HEIGHT-1));

    let (start, end) = board::find_borders(&board);
    println!("{},{};{},{}",start.0,start.1,end.0,end.1);
    println!("{:?}",char_map);

    gui::main(char_map, word_start_arrows);

    board::print_crossword(&board, start, end);
}

/*
fn placeWord(mut queue : VecDeque<String>) -> [[char; MAPSIZE]; MAPSIZE] {

    // board::print_crossword(&board, start, end);
    let mut arr : [[char; MAPSIZE];MAPSIZE] = [[' '; MAPSIZE];MAPSIZE];

    if let Some(word) = queue.pop_front(){
        let middle = Point {x : MAPSIZE as i32 /2, y : (MAPSIZE - word.len()) as i32 /2};
        place(word, middle, 1, &mut arr, Direction::DOWN);
    }

    while !queue.is_empty(){
        if let Some(word) = queue.pop_front(){

        }
    }

    arr
} 
*/