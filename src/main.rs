use std::path::{PathBuf};
use aisysproj::{create_state, ElevateMap};

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problem_f_09.txt");
    let mut state = create_state(&mut path);
    let mut st = ElevateMap::create(&mut state);
    let mut el = st.unwrap();
    el.move_cleaner('W');
    for i in el.map{
        println!("{}",i);
    }

}
