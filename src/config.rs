pub use std::collections::{HashMap, VecDeque};
pub use std::cmp::{min,max};
pub use rand::thread_rng;
pub use rand::seq::SliceRandom;

pub type BoardType = Vec<Vec<char>>;
pub type Location = (usize,usize);
pub type Quality = f32;
pub type LocationMap = HashMap<char,Vec<(Location,Quality)>>;
pub type LocationMapWithQuality = HashMap<char,Vec<(Location,Quality)>>;

pub const START_HEIGHT: usize = 16;
pub const START_WIDTH: usize = 16;
pub const SHUFFLE_COUNT: usize = 10;

