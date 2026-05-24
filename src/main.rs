mod board;
mod config;
// use config::*;
mod gui;

fn main() {
    // let (board,char_map, word_arrows) = board::return_crossword(&mut words);

    // board::print_crossword(&board, (0,0), (START_WIDTH-1,START_HEIGHT-1));

    // println!("{:?}",char_map);

    gui::main();

    // let (start, end) = board::find_borders(&board);
    // board::print_crossword(&board, start, end);
}