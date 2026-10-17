use crate::config::*;

pub fn return_crossword (words : &mut Vec<&str>) -> Option<(LocationMap,LocationMapWord)> {
    let mut smallest_board : ((i32,i32), (i32,i32));

    if let Some((mut _best_board,mut bb_char_map,mut best_arrows)) = generate_crossword(words){
        let ((a, b), (c, d)) = find_borders(bb_char_map.clone());
        smallest_board = ((a as i32, b as i32), (c as i32, d as i32));

        for _ in 0..SHUFFLE_COUNT {
            if let Some((cur_board, char_map, arrows)) = generate_crossword(words){
                let ((a, b), (c, d)) = find_borders(char_map.clone());
                let pos = ((a as i32, b as i32), (c as i32, d as i32));
                
                //find_borders -> start - right-uppper | end - left-lower
                let dist_best = (smallest_board.0.1 - smallest_board.1.1)*(smallest_board.0.1 - smallest_board.1.1) +
                    (smallest_board.0.0 - smallest_board.1.0)*(smallest_board.0.0 - smallest_board.1.0);
                let dist_cur = (pos.0.1 - pos.1.1)*(pos.0.1 - pos.1.1) +
                    (pos.0.0 - pos.1.0)*(pos.0.0 - pos.1.0);
                if dist_best > dist_cur {
                    smallest_board = pos;
                    _best_board = cur_board;
                    bb_char_map = char_map;
                    best_arrows = arrows;
                }
            }
        }    
        return Some((bb_char_map, best_arrows));
    } else {
        return None;
    }
}

fn map_drop_bonus<K>(old_map: HashMap<char, Vec<(Location,K)>>) -> LocationMap {
    old_map
        .into_iter() 
        // unpack to char, ver
        .map(|(letter, vec)| 
            {
                let new_vec: Vec<(usize, usize)> = vec.into_iter().map(|(loc, _)| loc).collect();
                (letter, new_vec)
            }
        )
        .collect()
}

pub fn generate_crossword(words : &mut Vec<&str>) -> Option<(BoardType,LocationMap,LocationMapWord)>{
    let mut attempts = 0;
    loop {
        attempts += 1;
        if attempts >= WORD_PLACE_TRY_LIMIT{
            return None;
        }
        let mut board: Vec<Vec<char>> = vec![vec![' '; START_WIDTH]; START_HEIGHT];
        let mut char_map: LocationMapQuality = HashMap::new();
        let mut arrows: LocationMapWord = HashMap::new();

        let mut counter = 0;

        words.shuffle(&mut thread_rng());
        let mut cur_words: VecDeque<&str> = words.iter().copied().collect();

        while !cur_words.is_empty() {
            let word = cur_words.pop_front().unwrap();

            if let Some(word_placement) = best_placement(&mut board, &mut char_map, word) {
                let pos = word_placement.1;
                let dir = word_placement.2;
                arrows.entry(dir).or_insert(Vec::new()).push((pos,word.to_string()));
                counter = 0;
            } else {
                counter+=1;
                cur_words.push_back(word);
            }

            if counter == cur_words.len(){
                break;
            }
        }
        if cur_words.is_empty(){
            return Some((board,map_drop_bonus(char_map), arrows)); 
        }
    }
}

fn best_placement(board: &mut BoardType, char_map: &mut LocationMapQuality, word: &str) -> Option<(Quality, Location, Direction)> {
    let length = word.chars().count() as usize;
    let mut best : Option<(Quality, Location, Direction)> = None;
    if char_map.is_empty() { // first word on board
        let start_x = (START_WIDTH / 2).saturating_sub(length/2);
        let start_y = START_HEIGHT / 2;
        let start: Location = (start_x, start_y);
        
        place_from_point(board, char_map, word, start, Direction::RIGHT) ;
        return Some((0.0,start,Direction::RIGHT));
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
        return best;
    }

    None
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
    let length = word.chars().count() as usize;

    match dir{
        Direction::DOWN => {
            if start.1 > height {
                return false
            }
            //no word above and under the word
            if start.1 > 0 && board[start.1 - 1][start.0] != ' ' {
                return false
            }
            if start.1 + length < height && board[start.1 + length][start.0] != ' ' {
                return false;
            }
        }
        Direction::RIGHT =>{
            if start.0 > width {
                return false
            }
            //no word right and left to word
            if start.0 > 0 && board[start.1][start.0 - 1] != ' ' {
                return false;
            }
            if start.0 + length < width && board[start.1][start.0 + length] != ' ' {
                return false;
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

fn place_from_point(board : &mut BoardType, char_map : &mut LocationMapQuality,
                    word : &str, mut start : Location, dir : Direction) {
    let length : f32 = word.chars().count() as f32;

    for (idx, letter) in word.chars().enumerate() {
        let quality = (length / 2.0 - idx as f32).abs() / (length/2.0);
        let location = char_map.entry(letter).or_default();


        if let Some(idx) = location.iter().position(|(loc, _)| *loc == start) {
            location.remove(idx);
            location.push((start, quality));
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
pub fn find_borders(char_map : HashMap<char,Vec<Location>>) -> (Location,Location){

    let first_location = char_map
        .values()
        .flatten()
        .next()
        .copied()
        .expect("Nothing found, can't find borders!");

    let mut start: (usize,usize) = first_location; 
    let mut end: (usize, usize) = first_location;

    for (_,locations) in char_map{
        for location in locations{
            start = (min(location.0, start.0), min(location.1, start.1));
            end = (max(location.0, end.0), max(location.1, end.1));
        }
    }
    return (start,end);
}

/// function that prints the crossword in certain range, including end point coordinates!
pub fn _print_crossword(board: &BoardType, start: Location, end: Location){
    for row in &board[start.1 .. end.1+1]{
        let slice = &row[start.0 .. end.0+1];   

        println!("{:?}",slice);
    }   
}

pub fn shift_char_map<K>(char_map : &mut HashMap<K,Vec<Location>>, x_shift : usize, y_shift : usize){
    for (_,locations) in char_map{
        for location in locations{
            location.0 -= x_shift;
            location.1 -= y_shift;
        }
    }
}

pub fn shift_arrows<K,T>(char_map : &mut HashMap<K,Vec<(Location,T)>>, x_shift : usize, y_shift : usize){
    for (_,words_info) in char_map{
        for (location,_) in words_info{
            location.0 -= x_shift;
            location.1 -= y_shift;
        }
    }
}