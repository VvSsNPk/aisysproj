use std::path::{PathBuf};
use aisysproj::{create_state, ElevateMap};

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problem_f_04.txt");
    let mut state = create_state(&mut path);
    let mut st = ElevateMap::create(&mut state);
    let mut el = st.unwrap();
    let x = el.find_plan();
    println!("answer is :   {}",x);

}
