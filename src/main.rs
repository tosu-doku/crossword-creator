mod input;
mod board;
mod config;
use config::*;
use std::collections::HashMap;
use fltk::{app::{self, screen_size}, enums::{Color,Font}, prelude::{GroupExt, TableExt, WidgetExt}, window};
use fltk_table::{SmartTable, TableOpts};

const START_HEIGHT: usize = 16;
const START_WIDTH: usize = 16;

type BoardType = Vec<Vec<char>>;
// type Location = (usize,usize);
// type LocationMap = HashMap<char,Vec<(usize,usize)>>;

fn main() {
    let mut board: BoardType = vec![vec![' '; START_WIDTH as usize]; START_HEIGHT as usize];

    // let mut char_map: HashMap<char,Vec<(usize,usize)>> = HashMap::new();
    
    let mut list_of_words: Vec<&str> = vec!["pies","konik","chomik"];
    board = board::return_crossword(&mut list_of_words);

    board::print_crossword(&board, (0,0), (START_WIDTH-1,START_HEIGHT-1));

    let (start, end) = board::find_borders(&board);
    println!("{},{};{},{}",start.0,start.1,end.0,end.1);
    println!("{:?}",char_map);

    // GUI
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/4.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/2.0) as i32;

    println!("{:?}",screen_size());

    let mut win = window::Window::default();
    win.set_size(width, height);
    win.make_resizable(true);

    let mut crossword_table = SmartTable::default()
        .size_of(&win)
        .center_of_parent()
        .with_opts(TableOpts {
            rows: START_HEIGHT as i32,
            cols: START_WIDTH as i32,
            cell_font: Font::by_index(1),
            cell_font_size: 20,
            editable: true,
            header_color: Color::Blue.lighter().lighter(),
            ..Default::default()
        });
    // crossword_table.set_color(Color::Black);
    crossword_table.set_row_height_all(40);
    crossword_table.set_col_width_all(40);
    crossword_table.end();

    let mut tmp = [0; 4]; // buffor for char -> &str conversion
    
    // map all values to crossword_table
    for elem  in char_map{
        let letter: &mut str = elem.0.encode_utf8(&mut tmp);
        for placement in elem.1{
            crossword_table.set_cell_value(placement.0 as i32, placement.1 as i32, letter);

        }
    }

    win.end(); 
    win.show();
    win.set_pos(x_pos, y_pos);
    my_app.run().unwrap();

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