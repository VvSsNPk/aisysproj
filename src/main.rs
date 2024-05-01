use std::path::Path;
use aisysproj::create_state;

pub mod state;

fn main() {
    let path = Path::new("problem_d_07.txt");
    let mut x = create_state(path);
    let u = x.find_plan();
    println!("{}",u)

}
