pub use std::collections::{HashMap, VecDeque};
pub use std::cmp::{min,max};
pub use rand::thread_rng;
pub use rand::seq::SliceRandom;

pub type BoardType = Vec<Vec<char>>;
pub type Location = (usize,usize);
pub type Quality = f32;
pub type LocationMap = HashMap<char,Vec<Location>>;
pub type LocationMapWord = HashMap<Direction,Vec<(Location,String)>>;
pub type LocationMapQuality = HashMap<char,Vec<(Location,Quality)>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    DOWN,
    RIGHT
}

pub const START_HEIGHT: usize = 32;
pub const START_WIDTH: usize = 32;
pub const SHUFFLE_COUNT: usize = 10;