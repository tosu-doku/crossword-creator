use std::collections::{HashMap, VecDeque};
use std::cmp::{min,max};
use std::fmt::Alignment::Right;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::board::Direction::RIGHT;

// export types to dependency.rs later
type BoardType = Vec<Vec<char>>;
type Location = (usize,usize);
type Quality = f32;
type LocationMap = HashMap<char,Vec<(Location,Quality)>>;

const START_HEIGHT: usize = 16;
const START_WIDTH: usize = 16;
const SHUFFLE_COUNT: usize = 10;

#[derive(Debug, Clone, Copy)]
enum Direction {
    DOWN,
    RIGHT
}

pub fn place(board: &mut BoardType, char_map: &mut LocationMap, word: &str) {
    for letter in word.chars(){
        if char_map.contains_key(&letter) {
            // FIXME - placeholder | implement: words crossing over                     
            println!("TODO");
            return;
        }
    }
    
    let height = board.len();
    let width = board[0].len();

    // if letters in word not found 
    let place_y = height/2;
    let mut place_x = width/2;
    for letter in word.chars(){

        // FIXME - placeholder | implement: find a free spot of word.len()
                 
        board[place_y][place_x] = letter;

        if let Some(locations) = char_map.get_mut(&letter) {
            locations.push((place_y,place_x));
        }
        else {
            char_map.insert(letter,vec![(place_y,place_x)]);
        }

        place_x+=1;
    }
}


pub fn return_crossword (list : &mut Vec<&str>){
    let mut best_board : Vec<Vec<char>>;
    let mut best_pos : (Location, Location);
    let mut count = 0;

    while count < SHUFFLE_COUNT{
        let cur_board = generate_crossword(list);
        let pos : (Location, Location) = find_borders(&cur_board);
        if count == 0 {
            best_board = cur_board;
            count += 1;
            best_pos = pos.clone();
            continue;
        }
        //find_borders -> start - right-uppper | end - left-lower
        let dist_best = (best_pos.0.1 - best_pos.1.1)*(best_pos.0.1 - best_pos.1.1) +
            (best_pos.0.0 - best_pos.1.0)*(best_pos.0.0 - best_pos.1.0);
        let dist_cur = (pos.0.1 - pos.1.1)*(pos.0.1 - pos.1.1) +
            (pos.0.0 - pos.1.0)*(pos.0.0 - pos.1.0);
        if dist_best < dist_cur {
            best_pos = pos;
            best_board = cur_board;
        }

        count += 1;
    }

}
pub fn generate_crossword(list : &mut Vec<&str>) -> Vec<Vec<char>>{
    let mut board : Vec<Vec<char>>;
    let mut char_map : HashMap<char,Vec<(Location,Quality)>>;

    while 1 {
        let mut pom :(&str, usize);
        let mut check = true;

        list.shuffle(&mut thread_rng());
        let mut cur= list.clone();
        while !cur.is_empty(){
            let mut word = cur.pop();
            if !best_placement(&mut board, &mut char_map, word){
                if Some(pom){
                    if pom.0 == word && cur.len() == pom.1 {
                        check = false;
                        break;
                    }
                }
                cur.push_front(word);
            }
        }
        if check{
            return board;
        }
    }
}
pub fn best_placement(board: &mut BoardType, char_map: &mut LocationMap, word: &str) -> bool {
    let length = word.chars().count() as usize;
    let mut best : Option<(Quality, Location, Direction)> = None;
    if char_map.is_empty() { // first word on board
        let lenght = (word.chars().count() as f32 / 2) as usize;
        let start: Location = (START_WIDTH/2 - lenght, START_HEIGHT / 2);
        place_from_point(board, char_map, word, start, RIGHT) ;

        return true;
    }

    for (idx, letter) in word.chars().enumerate() {
        if let Some(locations) = char_map.get(&letter) {
            if let Some(cur) = best_for_letter(board, locations, word, &letter, idx ) {
                if best.is_none() || cur.0 < best.clone().unwrap().0 {
                    best = Some(cur);
                }
            }
        }
    }

    if let Some((_, pos, dir)) = best{
        place_from_point(board, char_map, word, pos, dir);
        return true;
    }

    false
}
pub fn best_for_letter(board: &mut BoardType, char_map: &[(Location, f32)], word: &str,
                       letter :&char, idx : usize) -> Option<(f32, Location, Direction)>{
    let height = board.len();
    let width = board[0].len();
    let length = word.chars().count() as usize;

    let mut sorted_loc = char_map.to_vec();
    sorted_loc.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    for (loc, q) in sorted_loc{

        let dir = if (loc.0  > 0 && board[loc.0 - 1][loc.1] != ' ') ||
            ((loc.0 + 1) <= height && board[loc.0 + 1][loc.1] != ' '){
            Direction::RIGHT
        } else{
            Direction::DOWN
        };

        let start_loc = match dir{
            Direction::DOWN => {
                if loc.0 < idx { continue; }
                (loc.0 - idx, loc.1)
            },
            Direction::RIGHT => {
                if loc.1 < idx { continue; }
                (loc.0, loc.1 - idx)
            }
        };

        if can_place(board, word, idx, start_loc, dir){
            let quality = (length as f32 / 2.0 - idx as f32).abs() / (length as f32/2.0) as f32;
            return Some((q + quality, start_loc, dir));
        }

    }

    None

}

pub fn can_place(board: &mut BoardType, word: &str, idx : usize,
                 start : Location, dir: Direction) -> bool{
    let length = word.chars().count() as usize;
    let height = board.len() as usize;
    let width = board[0].len() as usize;

    match dir{
        Direction::DOWN => {
            if  start.0 < 0 || start.0 > height{
                return false
            }
        }
        Direction::RIGHT =>{
            if  start.1 < 0 || start.1 > width{
                return false
            }
        }
    }

    let mut location = start.clone();
    for (idx, letter) in word.chars().enumerate() {
        if board[location.0][location.1] != ' ' && board[location.0][location.1] != letter {
            return false;
        }
        match dir {
            Direction::DOWN => location.0 += 1,
            Direction::RIGHT => location.1 += 1,
        }
    }

    true
}

pub fn place_from_point(board : &mut BoardType, char_map : &mut LocationMap,
                    word : &str, mut start : Location, dir : Direction) {
    let length : f32 = word.chars().count() as f32;

    for (idx, letter) in word.chars().enumerate() {
        //TODO: delete cross letter and his neibours
        board[start.0][start.1] = letter;
        let quality = (length / 2.0 - idx as f32).abs() / (length/2.0);

        char_map.entry(letter).or_insert(Vec::new()).push((start,quality));

        match dir{
            Direction::DOWN => start.0 += 1,
            Direction::RIGHT => start.1 += 1,
            _ => {}
        }
    }
}

/// this function aims to return 'edge' points containing the part of a crossword that is filled
pub fn find_borders(board: &Vec<Vec<char>>) -> (Location,Location){

    let mut value_found: bool = false;
    let mut start: (usize,usize) = (board[0].len(),board.len()); 
    let mut end: (usize, usize) = (0,0);

    for row_idx in 0..board.len(){
        for col_idx in 0..board[0].len(){
            // skip if letter not found
            if board[row_idx][col_idx] == ' ' {
                continue;
            }
            else {
                value_found = true;
                start = (min(row_idx,start.0),min(col_idx,start.1));
                end = (max(row_idx,end.0),max(col_idx,end.1));
                println!("{:?},{:?}",start,end)
            }
        }
    }

    if !value_found {    // FIXME - TEMPORARY SOLUTION
        panic!("Nothing found, can't find borders!")
    }
    
    return (start,end);
}

/// function that prints the crossword in certain range, including end point coordinates!
pub fn print_crossword(board: &BoardType, start: Location, end: Location){

    for row in &board[start.0 .. end.0+1]{                  // delete horizontally
        let slice = &row[start.1 .. end.1+1];   

        println!("{:?}",slice);
    }   
}
