mod input;
mod board;
// use std::collections::{HashMap};
const START_HEIGHT: usize = 16;
const START_WIDTH: usize = 16;

type BoardType = Vec<Vec<char>>;

fn main() {
    let mut board: BoardType = vec![vec![' '; START_WIDTH as usize]; START_HEIGHT as usize];

    // let mut char_map: HashMap<char,Vec<(usize,usize)>> = HashMap::new();
    
    let mut list_of_words: Vec<&str> = vec!["pies","konik","chomik"];
    board = board::return_crossword(&mut list_of_words);

    board::print_crossword(&board, (0, 0), (START_WIDTH - 1, START_HEIGHT - 1));
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