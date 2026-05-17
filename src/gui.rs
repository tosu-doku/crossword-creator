use fltk::{app::{self, screen_size}, enums::{Color,Font}, prelude::{GroupExt, TableExt, WidgetExt}, window};
use fltk_table::{SmartTable, TableOpts};

use crate::config::*;


pub fn main(char_map : LocationMap){
    // create app
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/4.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/2.0) as i32;

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
    crossword_table.set_row_height_all(40);
    crossword_table.set_col_width_all(40);
    crossword_table.end();
    
    place_values(crossword_table, char_map);

    win.end(); 
    win.show();
    win.set_pos(x_pos, y_pos);
    my_app.run().unwrap();
}

/// map all values to crossword_table
pub fn place_values(mut crossword_table : SmartTable, char_map : LocationMap){

    let mut tmp = [0; 4]; // buffor for char -> &str conversion

    for elem in char_map{
        let letter: &mut str = elem.0.encode_utf8(&mut tmp);
        for placement in elem.1{
            crossword_table.set_cell_value(placement.1 as i32, placement.0 as i32, letter);
        }
    }
}