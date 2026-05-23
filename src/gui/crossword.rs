use fltk::{draw, enums::{Color,Font, FrameType}, frame, group, prelude::*, widget::Widget, window};
use fltk_table::{SmartTable, TableOpts};

use crate::config::*;

pub const CELL_SIZE: i32 = 40;
pub const HEADER_OFFSET : i32 = CELL_SIZE;

pub const DIRECTION_HEADER_HEIGHT : i32 = 20;
pub const DIRECTION_HEADER_WIDTH : i32 = 100;
pub const DIRECTION_HEADER_FONT_SIZE : i32 = 16;

pub const HINT_HEIGHT : i32 = 20;
pub const HINT_WIDTH : i32 = 100;

pub const NUMBER_FONT_SIZE : i32 = 18;


pub fn show_crossword(win : &window::DoubleWindow, char_map : LocationMap, arrows : LocationMapWord, hints_for_words : HashMap<String,String>){
    let mut crossword_table = SmartTable::default()
        .size_of(win)
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

    draw_arrows_numbers(&crossword_table, arrows.clone());

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

    _hints_wrapper.end();

    place_values(crossword_table, char_map);
}


fn draw_rect_right(cell_x: i32,  cell_y: i32){
    let mut triangle_cell = Widget::new(cell_x+1, cell_y, CELL_SIZE, CELL_SIZE, None);

    triangle_cell.draw(|w| {
        draw::set_line_style(draw::LineStyle::Solid, 2);
        draw::set_draw_color(Color::Black);

        // triangle
        draw::begin_line();
        draw::vertex(w.x() as f64, w.y() as f64);                                     // upper left corner
        draw::vertex((w.x() + w.w() / 5) as f64, (w.y() + w.h() / 2) as f64);
        draw::vertex(w.x() as f64, (w.y() + w.h() - 1) as f64);                     // lower left corner
        draw::vertex(w.x() as f64, w.y() as f64);                                     // upper left corner

        draw::end_line();
        
        draw::set_line_style(draw::LineStyle::Solid, 1);
    });
}

fn draw_rect_down(cell_x: i32, cell_y: i32){
    let mut triangle_cell = Widget::new(cell_x, cell_y-1, CELL_SIZE, CELL_SIZE, None);

    triangle_cell.draw(|w| {
        draw::set_line_style(draw::LineStyle::Solid, 2);
        draw::set_draw_color(Color::Black);
        
        // triangle
        draw::begin_line();
        draw::vertex(w.x() as f64, w.y() as f64);                                   // upper left corner
        draw::vertex((w.x() + w.w() / 2) as f64, (w.y() + w.h() / 5) as f64); 
        draw::vertex((w.x()+w.w()) as f64, w.y() as f64);                        // upper right corner
        draw::vertex(w.x() as f64, w.y() as f64);                                   // upper left corner
        draw::end_line();
        
        draw::set_line_style(draw::LineStyle::Solid, 1);
    });
}


/// map all values to crossword_table
fn place_values(mut crossword_table : SmartTable, char_map : LocationMap){

    let mut tmp = [0; 4]; // buffor for char -> &str conversion

    for elem in char_map{
        let letter: &mut str = elem.0.encode_utf8(&mut tmp);
        for placement in elem.1{
            crossword_table.set_cell_value(placement.1 as i32, placement.0 as i32, letter);
        }
    }
}

fn draw_arrows_numbers(crossword_table : &SmartTable, arrows : LocationMapWord){
    for (dir,words_info) in arrows{
        let locations : Vec<Location> = words_info.iter().map(|(loc, _)| *loc).collect();
        for (mut idx,location) in locations.iter().enumerate(){
            idx+=1;
            let cell_x: i32 = crossword_table.x() + HEADER_OFFSET + (location.0 as i32  * CELL_SIZE);
            let cell_y: i32 = crossword_table.y() + HEADER_OFFSET/2 + (location.1 as i32 * CELL_SIZE); 
            match dir {
                Direction::DOWN => {
                        draw_rect_down(cell_x,cell_y);
                        frame::Frame::new(cell_x + CELL_SIZE * 3/4 - 1, cell_y + CELL_SIZE * 3/5, 10, 10, 
                            format!("{idx}").as_str()).set_label_size(NUMBER_FONT_SIZE);
                },
                Direction::RIGHT => {
                        draw_rect_right(cell_x,cell_y);
                        frame::Frame::new(cell_x + CELL_SIZE * 3/4 - 1, cell_y + 3, 10, 10, 
                            format!("{idx}").as_str()).set_label_size(NUMBER_FONT_SIZE);
                }
            }
        }
    }
}