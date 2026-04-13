use std::collections::HashMap;

// export types to dependency.rs later
type BoardType = Vec<Vec<char>>;
type Location = (usize,usize);
type LocationMap = HashMap<char,Vec<(usize,usize)>>;

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

/// this function aims to return 'edge' points containing the part of a crossword that is filled
pub fn find_borders(board: &Vec<Vec<char>>) -> (Location,Location){

    let mut value_found: bool = false;
    let mut start: (usize,usize) = (0,0); 
    let mut end: (usize, usize) = (0,0);

    for row_idx in 0..board.len(){
        for col_idx in 0..board[0].len(){
            // skip if letter not found
            if board[row_idx][col_idx] == ' ' {
                continue;
            }
            // mark the first appearance
            else if !value_found {
                start = (row_idx,col_idx);
                value_found = true
            }
            // mark the last appearance
            end = (row_idx,col_idx);
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
