mod board;
mod config;
// use config::*;
mod gui;

fn main() {
    // let mut words: Vec<&str> = vec!["pies","konik","chomik","ptak","kot"];

    // let hints: Vec<&str> = vec!["robi hau","konik","chomik","ptak","kot"];

    // let hints_for_words: HashMap<String, String> = words.iter().zip(hints.iter()).map(|(&w, &h)| (w.to_string(), h.to_string())).collect();

    // println!("{:?}",words_with_hints);

    // let (board,char_map, word_arrows) = board::return_crossword(&mut words);

    // board::print_crossword(&board, (0,0), (START_WIDTH-1,START_HEIGHT-1));

    // println!("{:?}",char_map);

    gui::main();

    // let (start, end) = board::find_borders(&board);
    // board::print_crossword(&board, start, end);
}