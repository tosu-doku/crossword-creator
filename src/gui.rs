use fltk::{app::{self, screen_size}, draw, enums::{Color,Font, FrameType}, frame, group, prelude::*, widget::Widget, window};
use fltk_table::{SmartTable, TableOpts};

use crate::config::*;

pub const DIRECTION_HEADER_HEIGHT : i32 = 20;
pub const DIRECTION_HEADER_WIDTH : i32 = 100;
pub const DIRECTION_HEADER_FONT_SIZE : i32 = 16;

pub const HINT_HEIGHT : i32 = 20;
pub const HINT_WIDTH : i32 = 100;


pub fn main(char_map : LocationMap, arrows : LocationMapWord, hints_for_words : HashMap<&str,&str>){
    // create app
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/6.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/1.5) as i32;

    let mut win = window::Window::default();
    win.set_size(width, height);
    // win.make_resizable(true);

    let mut crossword_table = SmartTable::default()
        .size_of(&win)
        // .with_size(win.width()-100, win.height())
        .center_of_parent()
        .with_opts(TableOpts {
            rows: START_HEIGHT as i32,
            cols: START_WIDTH as i32,
            cell_font: Font::by_index(1),
            cell_font_size: 20,
            // editable: true,
            header_color: Color::Blue.lighter().lighter(),
            // header_frame: FrameType::UpBox,
            ..Default::default()
        });
    crossword_table.set_row_height_all(CELL_SIZE);
    crossword_table.set_col_width_all(  CELL_SIZE);
    crossword_table.end();

    // redraw boxes on mouseEvents
    let mut win_clone = win.clone();
    crossword_table.handle(move |_, ev| {
        match ev {
            fltk::enums::Event::Push | fltk::enums::Event::Released | 
            fltk::enums::Event::KeyDown | fltk::enums::Event::MouseWheel => {
                win_clone.redraw(); 
                true
            }
            _ => false,
        }
    });

    draw_arrows(&crossword_table, arrows.clone());

    // word hints
    let hints_width = 200;
    let mut _hints_wrapper = group::Pack::new(crossword_table.width()-hints_width, 0,hints_width,200,"hints");

    for (dir,words_info) in arrows{
        let words : Vec<String> = words_info.into_iter().map(|(_, w)| w).collect();

        let mut direction_header = match dir {
            Direction::DOWN => frame::Frame::new(0, 0, DIRECTION_HEADER_WIDTH, DIRECTION_HEADER_HEIGHT, "PIONOWO"),
            Direction::RIGHT => frame::Frame::new(0, 0, DIRECTION_HEADER_WIDTH, DIRECTION_HEADER_HEIGHT, "POZIOMO")
        };
        direction_header.set_frame(FrameType::BorderBox);
        direction_header.set_label_size(DIRECTION_HEADER_FONT_SIZE);
        direction_header.set_color(Color::from_rgb(185, 156, 230));

        for (mut idx,word) in words.iter().enumerate(){
            if let Some(hint_for_word) = hints_for_words.get(word.as_str()){
                idx+=1;
                let hint: String = format!("{idx} - {hint_for_word}");
                let mut frame = frame::Frame::new(0, 0, HINT_WIDTH, HINT_HEIGHT, hint.as_str());
                frame.set_frame(FrameType::FlatBox);
                frame.set_color(Color::Blue.lighter().lighter().lighter());
            }
        }
        frame::Frame::new(0, 0, DIRECTION_HEADER_WIDTH, DIRECTION_HEADER_HEIGHT, ""); //spacing between hint directions
    }

    println!("{:?}",crossword_table.width());

    _hints_wrapper.end();

    place_values(crossword_table, char_map);

    win.end(); 
    win.show();
    win.set_pos(x_pos, y_pos);
    my_app.run().unwrap();
}

fn draw_rect_right(crossword_table : &SmartTable, x: i32,  y: i32){
    let header_offset = CELL_SIZE; 
    let cell_x = crossword_table.x() + header_offset + (x * CELL_SIZE);
    let cell_y = crossword_table.y() + header_offset/2 + (y * CELL_SIZE); 

    let mut triangle_cell = Widget::new(cell_x+1, cell_y, CELL_SIZE, CELL_SIZE, None);

    triangle_cell.draw(|w| {
        draw::set_line_style(draw::LineStyle::Solid, 2);
        draw::set_draw_color(Color::Black);

        // triangle
        draw::begin_line();
        draw::vertex(w.x() as f64, w.y() as f64);                                     // upper left corner
        draw::vertex((w.x() + w.w() / 4) as f64, (w.y() + w.h() / 2) as f64);
        draw::vertex(w.x() as f64, (w.y() + w.h()) as f64);                         // lower left corner
        draw::vertex(w.x() as f64, w.y() as f64);                                     // upper left corner

        draw::end_line();
        
        draw::set_line_style(draw::LineStyle::Solid, 1);
    });
}

fn draw_rect_down(crossword_table : &SmartTable, x: i32,  y: i32){
    let header_offset = CELL_SIZE; 
    let cell_x = crossword_table.x() + header_offset + (x * CELL_SIZE);
    let cell_y = crossword_table.y() + header_offset/2 + (y * CELL_SIZE); 

    let mut triangle_cell = Widget::new(cell_x, cell_y-1, CELL_SIZE, CELL_SIZE, None);

    triangle_cell.draw(|w| {
        draw::set_line_style(draw::LineStyle::Solid, 2);
        draw::set_draw_color(Color::Black);
        
        // triangle
        draw::begin_line();
        draw::vertex(w.x() as f64, w.y() as f64);                                   // upper left corner
        draw::vertex((w.x() + w.w() / 2) as f64, (w.y() + w.h() / 4) as f64); 
        draw::vertex((w.x()+w.w()) as f64, w.y() as f64);                        // upper right corner
        draw::vertex(w.x() as f64, w.y() as f64);                                   // upper left corner
        draw::end_line();
        
        draw::set_line_style(draw::LineStyle::Solid, 1);
    });
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

fn draw_arrows(crossword_table : &SmartTable, arrows : LocationMapWord){
    for (dir,words_info) in arrows{
        let locations : Vec<Location> = words_info.iter().map(|(loc, _)| *loc).collect();
        match dir {
            Direction::DOWN => {
                for location in locations{
                    draw_rect_down(crossword_table,location.0 as i32,location.1 as i32);
                }
            },
            Direction::RIGHT => {
                for location in locations{
                    draw_rect_right(crossword_table,location.0 as i32,location.1 as i32);
                }
            }
        }
    }
}