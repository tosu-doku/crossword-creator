mod input;
mod board;
mod config;
use config::*;
mod gui;

fn main() {
    let mut words: Vec<&str> = vec!["pies","konik","chomik","ptak","kot"];

    let hints: Vec<&str> = vec!["robi hau","konik","chomik","ptak","kot"];

    let words_with_hints: Vec<(&str, &str)>= words.iter().zip(hints.iter()).map(|(&w, &h)| (w, h)).collect();

    // println!("{:?}",words_with_hints);

    let (board,char_map, word_arrows) = board::return_crossword(&mut words);

    board::print_crossword(&board, (0,0), (START_WIDTH-1,START_HEIGHT-1));

    println!("{:?}",char_map);

    gui::main(char_map, word_arrows);

    // let (start, end) = board::find_borders(&board);
    // board::print_crossword(&board, start, end);
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