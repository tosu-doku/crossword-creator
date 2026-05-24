use std::{rc::Rc,cell::RefCell};
use fltk::{app::{self, screen_size}, button, enums, frame, group, input, prelude::*, window};

mod crossword;
use crate::{board::{self, find_borders, shift_arrows, shift_char_map}, config::*, gui::crossword::{place_values, show_crossword}};

// crossword options
pub const CELL_SIZE: i32 = 40;
pub const HEADER_OFFSET : i32 = CELL_SIZE;

pub const DIRECTION_HEADER_HEIGHT : i32 = 20;
pub const DIRECTION_HEADER_WIDTH : i32 = 100;
pub const DIRECTION_HEADER_FONT_SIZE : i32 = 16;

const HINT_HEIGHT : i32 = 20;
const HINT_WIDTH : i32 = 100;
const HINTS_HEIGHT : i32 = 500;
const HINTS_WIDTH : i32 = 250;
const SEPERATE_HINTS_SPACE : i32 = 50;

pub const NUMBER_FONT_SIZE : i32 = 18;

const INPUT_BOX_WIDTH : i32 = 300;
const INPUT_BOX_HEIGHT : i32 = 80;

pub fn main(){
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/6.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/1.5) as i32;

    let mut win: window::DoubleWindow = window::Window::default();
    win.set_size(width, height);
    win.make_resizable(true);

    let mut input_wrapper = group::Group::default().size_of(&win);

    let mut input_box = input::Input::new(x_pos-INPUT_BOX_WIDTH/2, y_pos-INPUT_BOX_HEIGHT, INPUT_BOX_WIDTH, INPUT_BOX_HEIGHT,"");

    let mut input_description = frame::Frame::new(x_pos - INPUT_BOX_WIDTH/2, y_pos - INPUT_BOX_HEIGHT-70, 300, 60, "");
    input_description.set_label("If you want to add word to crossword, \nwrite: word [SPACE] hint");
    input_description.set_label_font(enums::Font::HelveticaBold);

    let mut add_button = button::Button::new(x_pos-125, y_pos, 100, 40, "Add word");
    let mut start_button = button::Button::new(x_pos+25, y_pos, 100, 40, "Confirm");
    let mut notification = frame::Frame::new(x_pos - INPUT_BOX_WIDTH/2, y_pos + 60, 300, 30, "");
    notification.set_label_color(enums::Color::from_rgb(50, 100, 150));

    let mut world_list = frame::Frame::new(x_pos - INPUT_BOX_WIDTH/2, y_pos + 130, 300, 60, "");
    world_list.set_label("World list: ");
    world_list.set_align(enums::Align::Top | enums::Align::Center);

    input_wrapper.end();

    let mut crossword_wrapper = group::Group::default().size_of(&win);

    crossword_wrapper.hide();
    crossword_wrapper.end();

    win.end(); 
    win.show();
    win.set_pos(x_pos, y_pos);

    // Rc - let multiple owners | RefCall - allow for modification in deeper scope 
    let input_words = Rc::new(RefCell::new(Vec::<String>::new()));
    let input_hints = Rc::new(RefCell::new(Vec::<String>::new()));

    let input_words_c1 = Rc::clone(&input_words);
    let input_hints_c1 = Rc::clone(&input_hints);
    let mut notification_clone = notification.clone();
    let mut world_list_clone = world_list.clone();

    let mut add_button_clone = add_button.clone();

    input_box.handle(move |_, ev| {
        match ev {
            enums::Event::KeyDown => {
                if app::event_key() == enums::Key::Enter {
                    add_button_clone.do_callback();
                    true 
                } else {
                    false 
                }
            }
            _ => false
        }
    });

    add_button.set_callback(move |_| {
        if let Some((input_word, input_hint)) = input_box.value().split_once(' '){
            let word = input_word.trim();
            let hint = input_hint.trim();

            if word.is_empty() || hint.is_empty() {
                notification_clone.set_label_color(enums::Color::from_rgb(150, 0, 0));
                notification_clone.set_label(&format!("No text in hint or word."));
                panic!("no text in hint");
            }
            if word.chars().count() == 1{
                notification_clone.set_label_color(enums::Color::from_rgb(150, 0, 0));
                notification_clone.set_label(&format!("Word cannot be one letter."));
                panic!("too short word");
            }

            let word_len = word.len(); 
            let input_hint = format!("{input_hint} ({word_len})");

            // println!("{:?} {:?}",input_word,input_hint);
            input_words_c1.borrow_mut().push(input_word.to_string());
            input_hints_c1.borrow_mut().push(input_hint.to_string());

            notification_clone.set_label_color(enums::Color::from_rgb(50, 100, 150));
            notification_clone.set_label(&format!("Added: {}", input_word));

            let current_words = input_words_c1.borrow();
            let words_string = current_words.join(", ");
            world_list_clone.set_label(&format!("World list:\n{}", words_string));
            world_list_clone.redraw()
        } else {
            notification_clone.set_label_color(enums::Color::from_rgb(150, 0, 0));
            notification_clone.set_label(&format!("No space between word and hint."));
            panic!("no space between word and hint");
        }

        input_box.set_value("")
    });

    let input_words_c2 = Rc::clone(&input_words);
    let input_hints_c2 = Rc::clone(&input_hints);
    let mut notification_clone2 = notification.clone();

    start_button.set_callback(move |_| {
        let words = input_words_c2.borrow();
        let hints = input_hints_c2.borrow();
        let hints_for_words: HashMap<String, String> = words
                        .iter()
                        .zip(hints.iter())
                        .map(|(w, h)| (w.to_string(), h.to_string()))
                        .collect();

        println!("{:?}",hints_for_words);
        
        let mut words_to_place: Vec<&str> = words.iter().map(|s| s.as_str()).collect();

        if let Some((mut char_map,mut arrows)) = board::return_crossword(&mut words_to_place){
            let (mut lower_border, mut upper_border) = find_borders(char_map.clone());
            // spacing on left
            lower_border.0 -= 1;
            lower_border.1 -= 1;

            // spacing on right
            upper_border.0 += 2;
            upper_border.1 += 2;

            shift_char_map(&mut char_map, lower_border.0, lower_border.1);
            shift_arrows(&mut arrows, lower_border.0, lower_border.1);

            let mut board_size_x = upper_border.0 - lower_border.0;
            let mut board_size_y = upper_border.1 - lower_border.1;
            // minimal board size
            board_size_x = max(board_size_x,MINIMAL_BOARD_SIZE);
            board_size_y = max(board_size_y,MINIMAL_BOARD_SIZE);

            let new_width = HINTS_WIDTH + SEPERATE_HINTS_SPACE + HEADER_OFFSET + CELL_SIZE * board_size_x as i32;
            let new_height = HEADER_OFFSET + CELL_SIZE * board_size_y as i32;

            win.resize( win.x(),win.y(),new_width, new_height);
            win.make_resizable(false);

            let mut crossword_wrapper2 = crossword_wrapper.clone();
            crossword_wrapper2.resize(0, 0, new_width, new_height);

            crossword_wrapper2.begin();
            let crossword_table = show_crossword(&win, char_map.clone(), arrows, hints_for_words, board_size_x as i32, board_size_y as i32);
            let mut show_button = button::Button::new(new_width-HINTS_WIDTH+40, new_height-50, HINTS_WIDTH-50, 40, "Show answers");

            let crossword_table2 = crossword_table.clone();
            let char_map2 = char_map.clone();

            show_button.set_callback(move |_| { 
                place_values(crossword_table2.clone(), char_map2.clone());
                app::redraw();
            });
            crossword_wrapper2.end();

            input_wrapper.hide();

            crossword_wrapper2.show();
        } else {
            notification_clone2.set_label_color(enums::Color::from_rgb(255, 0, 0));
            notification_clone2.set_label(&format!("Not all words can be placed"));
            panic!("Not all words can be placed");
        }

        win.show();
        app::redraw();
    });

    my_app.run().unwrap();
}