use std::path::{PathBuf};
use aisysproj::{create_state};

pub mod state;
pub mod elevate;

fn main() {
    let mut path = PathBuf::new();
    path.push("problem_f_09.txt");
    let mut state = create_state(&mut path);
    println!("{:?}",state.moves);
    let start = ElevateMap::create(&mut state);
    println!("{}",start.unwrap());
}
