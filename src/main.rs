use std::path::{PathBuf};
use aisysproj::{create_state, ElevateMap};

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problem_f_09.txt");
    let mut state = create_state(&mut path);
    println!("{}",state.uncleaned.len());
    let st = ElevateMap::create(&mut state);
}
