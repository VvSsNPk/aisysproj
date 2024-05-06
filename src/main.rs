use std::path::{PathBuf};
use aisysproj::{create_state, directory_parser, ElevateMap};
use crate::state::point::Point;

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("example-problems");
    let dir = directory_parser(&mut path);

}

