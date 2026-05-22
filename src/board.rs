use crate::config::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    DOWN,
    RIGHT
}

pub fn return_crossword (list : &mut Vec<&str>) -> BoardType {
    let mut best_board : Vec<Vec<char>>;
    let mut best_pos : ((i32,i32), (i32,i32));

    best_board = generate_crossword(list);
    let ((a, b), (c, d)) = find_borders(&best_board);
    best_pos = ((a as i32, b as i32), (c as i32, d as i32));

    for _ in 0..SHUFFLE_COUNT {
        let cur_board: Vec<Vec<char>> = generate_crossword(list);
        let ((a, b), (c, d)) = find_borders(&best_board);
        let pos = ((a as i32, b as i32), (c as i32, d as i32));
        
        //find_borders -> start - right-uppper | end - left-lower
        let dist_best = (best_pos.0.1 - best_pos.1.1)*(best_pos.0.1 - best_pos.1.1) +
            (best_pos.0.0 - best_pos.1.0)*(best_pos.0.0 - best_pos.1.0);
        let dist_cur = (pos.0.1 - pos.1.1)*(pos.0.1 - pos.1.1) +
            (pos.0.0 - pos.1.0)*(pos.0.0 - pos.1.0);
        if dist_best < dist_cur {
            best_pos = pos;
            best_board = cur_board;
        }
    }

    return best_board;
}

pub fn generate_crossword(list : &mut Vec<&str>) -> Vec<Vec<char>>{
    loop {
        let mut board: Vec<Vec<char>> = vec![vec![' '; START_WIDTH]; START_HEIGHT];
        let mut char_map: HashMap<char, Vec<(Location, Quality)>> = HashMap::new();

        let mut counter = 0;

        list.shuffle(&mut thread_rng());
        let mut cur: VecDeque<&str> = list.iter().copied().collect();

        while !cur.is_empty() {
            let word = cur.pop_front().unwrap();

            if !best_placement(&mut board, &mut char_map, word) {
                counter+=1;
                cur.push_back(word);
            } else{
                counter = 0;
            }

            if counter == cur.len(){
                break;
            }
        }
        if cur.is_empty(){
            return board;
        }
    }
}

fn best_placement(board: &mut BoardType, char_map: &mut LocationMap, word: &str) -> bool {
    let length = word.chars().count() as usize;
    let mut best : Option<(Quality, Location, Direction)> = None;
    if char_map.is_empty() { // first word on board
        let start_x = (START_WIDTH / 2).saturating_sub(length/2);
        let start: Location = (start_x, START_HEIGHT / 2);
        
        place_from_point(board, char_map, word, start, Direction::RIGHT) ;
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

fn best_for_letter(board: &mut BoardType, char_map: &[(Location, f32)], word: &str,
                       _letter :&char, idx : usize) -> Option<(f32, Location, Direction)>{
    let height = board.len();
    // let width = board[0].len();
    let length = word.chars().count() as usize;

    let mut sorted_loc = char_map.to_vec();
    sorted_loc.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    for (loc, q) in sorted_loc{

        let dir = if (loc.1 > 0 && board[loc.1 - 1][loc.0] != ' ') ||
            ((loc.1 + 1) < height && board[loc.1 + 1][loc.0] != ' '){
            Direction::RIGHT
        } else{
            Direction::DOWN
        };

        let start_loc = match dir{
            Direction::DOWN => {
                if loc.1 < idx { continue; }
                (loc.0, loc.1 - idx)
            },
            Direction::RIGHT => {
                if loc.0 < idx { continue; }
                (loc.0 - idx, loc.1)
            }
        };

        if can_place(board, word, idx, start_loc, dir){
            let quality = (length as f32 / 2.0 - idx as f32).abs() / (length as f32/2.0) as f32;
            return Some((q + quality, start_loc, dir));
        }
    }

    None
}

fn can_place(board: &mut BoardType, word: &str, _idx : usize,
                 start : Location, dir: Direction) -> bool{
    //let length = word.chars().count() as usize;
    let height = board.len() as usize;
    let width = board[0].len() as usize;

    match dir{
        Direction::DOWN => {
            if start.1 > height {
                return false
            }
        }
        Direction::RIGHT =>{
            if start.0 > width {
                return false
            }
        }
    }

    let mut location = start.clone();
    for (_idx, letter) in word.chars().enumerate() {
        if board[location.1][location.0] != ' ' && board[location.1][location.0] != letter {
            return false;
        }
        match dir {
            Direction::DOWN => location.1 += 1,
            Direction::RIGHT => location.0 += 1,
        }
    }

    true
}

fn place_from_point(board : &mut BoardType, char_map : &mut LocationMap,
                    word : &str, mut start : Location, dir : Direction) {
    let length : f32 = word.chars().count() as f32;

    for (idx, letter) in word.chars().enumerate() {
        let quality = (length / 2.0 - idx as f32).abs() / (length/2.0);
        let location = char_map.entry(letter).or_default();


        if let Some(idx) = location.iter().position(|(loc, _)| *loc == start) {
            location.remove(idx);
        } else{
            location.push((start, quality));
        }
        board[start.1][start.0] = letter;

        match dir{
            Direction::DOWN => start.1 += 1,
            Direction::RIGHT => start.0 += 1,
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
                start = (min(col_idx, start.0), min(row_idx, start.1));
                end = (max(col_idx, end.0), max(row_idx, end.1));
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
    for row in &board[start.1 .. end.1+1]{
        let slice = &row[start.0 .. end.0+1];   

        println!("{:?}",slice);
    }   
}