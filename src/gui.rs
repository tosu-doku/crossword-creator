use fltk::{app::{self, screen_size}, draw, enums::{Color,Font}, prelude::*, widget::Widget, window};
use fltk_table::{SmartTable, TableOpts};

use crate::config::*;

pub fn main(char_map : LocationMap, arrows : LocationMapWord){
    // create app
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/4.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/2.0) as i32;

    let mut win = window::Window::default();
    win.set_size(width, height);
    // win.make_resizable(true);

    let mut crossword_table = SmartTable::default()
        .size_of(&win)
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
    
    draw_arrows(&crossword_table, arrows);

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