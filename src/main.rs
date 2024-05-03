use std::path::{PathBuf};
use aisysproj::{create_state, directory_parser};

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problem_f_09.txt");
    let mut state = create_state(&mut path);
    println!("{:?}",state.moves)
}
