use std::path::{PathBuf};
use aisysproj::{create_state, ElevateMap};

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problem_f_04.txt");
    let mut state = create_state(&mut path);
    let mut st = ElevateMap::create(&mut state);
    let mut el = st.unwrap();
    for i in "ENNNSSSWESNWEENNWNSSEESWWNNSESSWWESNNEES".chars(){
        el.move_cleaner(i)
    }
    for j in el.map{
        println!("{}",j);
    }
}
