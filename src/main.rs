use std::path::{Path};
use aisysproj::create_state;

pub mod state;

fn main() {
/*    let path = Path::new("problem_e_19.txt");
    let mut x = create_state(path);
    let y = x.find_plan();
    println!("{}",y.unwrap());*/

    let f = std::path::Path::read_dir(Path::new("."));
    for x in f.unwrap(){
        println!("{:?}",x.unwrap().path());
    }

}
