use fltk::{app::{self, screen_size}, prelude::*, window};

mod crossword;
use crate::{config::*, gui::crossword::show_crossword};

pub fn main(char_map : LocationMap, arrows : LocationMapWord, hints_for_words : HashMap<String,String>){
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/6.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/1.5) as i32;

    let mut win: window::DoubleWindow = window::Window::default();
    win.set_size(width, height);
    // win.make_resizable(true);

    show_crossword(&win, char_map, arrows, hints_for_words);

    win.end(); 
    win.show();
    win.set_pos(x_pos, y_pos);
    my_app.run().unwrap();
}
