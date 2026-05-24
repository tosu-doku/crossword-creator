use std::rc::Rc;
use std::cell::RefCell;
use fltk::{app::{self, screen_size}, button, enums, frame, group, input, prelude::*, window};

mod crossword;
use crate::{config::*, board, gui::crossword::show_crossword};

pub fn main(){
    let my_app: app::App = app::App::default().with_scheme(app::Scheme::Base);
    
    let x_pos: i32 = ((screen_size().0)/4.0) as i32;
    let y_pos: i32 = ((screen_size().1)/6.0) as i32;
    let width: i32 = ((screen_size().0)/2.0) as i32;
    let height: i32 = ((screen_size().1)/1.5) as i32;

    let mut win: window::DoubleWindow = window::Window::default();
    win.set_size(width, height);
    // win.make_resizable(true);

    let mut input_wrapper = group::Group::default().size_of(&win);

    let mut input_box = input::Input::new(x_pos-100, y_pos-80, 300, 80,"");

    let mut input_description = frame::Frame::new(x_pos - 100, y_pos - 130, 300, 60, "");;
    input_description.set_label("If you want to add word to crossword, \nwrite: [WORD] [SPACE] [HINT]");
    input_description.set_label_font(enums::Font::HelveticaBold);

    let mut add_button = button::Button::new(x_pos-50, y_pos, 100, 40, "Dodaj słowo");
    let mut start_button = button::Button::new(x_pos+50, y_pos, 100, 40, "Zatwierdź");
    let mut notification = frame::Frame::new(x_pos - 100, y_pos + 60, 300, 30, "");
    notification.set_label_color(enums::Color::from_rgb(50, 100, 150));

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
    let input_description = notification.clone();

    let input_words_c1 = Rc::clone(&input_words);
    let input_hints_c1 = Rc::clone(&input_hints);
    let mut notification_clone = notification.clone();
    // let mut input_box_clone = input_box.clone();

    add_button.set_callback(move |_| {
        // println!("{:?}",input_box.value());

        if let Some((input_word, input_hint)) = input_box.value().split_once(' '){
            let word = input_word.trim();
            let hint = input_hint.trim();

            if word.is_empty() || hint.is_empty() {
                notification_clone.set_label_color(enums::Color::from_rgb(50, 100, 150));
                notification_clone.set_label(&format!("No text in hint or word."));
                panic!("no text in hint");
            }

            // println!("{:?} {:?}",input_word,input_hint);
            input_words_c1.borrow_mut().push(input_word.to_string());
            input_hints_c1.borrow_mut().push(input_hint.to_string());

            notification_clone.set_label_color(enums::Color::from_rgb(50, 100, 150));
            notification_clone.set_label(&format!("Added: {}", input_word));
        } else {
            notification_clone.set_label_color(enums::Color::from_rgb(50, 100, 150));
            notification_clone.set_label(&format!("No space between word and hint."));
            panic!("no space between word and hint");
        }

        input_box.set_value("")
    });

    let input_words_c2 = Rc::clone(&input_words);
    let input_hints_c2 = Rc::clone(&input_hints);

    start_button.set_callback(move |_| {
        let mut words = input_words_c2.borrow();
        let hints = input_hints_c2.borrow();
        let hints_for_words: HashMap<String, String> = words
                        .iter()
                        .zip(hints.iter())
                        .map(|(w, h)| (w.to_string(), h.to_string()))
                        .collect();

        println!("{:?}",hints_for_words);
        
        let mut words_to_place: Vec<&str> = words.iter().map(|s| s.as_str()).collect();

        let (board,char_map, arrows) = board::return_crossword(&mut words_to_place);

        let win2 = win.clone();

        let mut crossword_wrapper2 = crossword_wrapper.clone();

        crossword_wrapper2.begin();
        show_crossword(&win2, char_map, arrows, hints_for_words);
        crossword_wrapper2.end();

        input_wrapper.hide();

        crossword_wrapper2.show();
    });

    my_app.run().unwrap();
}