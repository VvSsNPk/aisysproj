use std::path::Path;
use aisysproj::create_state;

pub mod state;

fn main() {
    let path = Path::new("problem_e_11.txt");
    let mut x = create_state(path);
    let y = x.find_plan();
    println!("{}",y);

}
