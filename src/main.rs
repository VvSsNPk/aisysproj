use std::path::Path;
use aisysproj::create_state;

pub mod state;

fn main() {
    let path = Path::new("problem_a_11.txt");
    let mut x = create_state(path);
 let mut str = match &x.moves{
        None => panic!("No string"),
        Some(x) => x.clone(),
    };
    for i in str.chars(){
        x.move_cleaner(i);
    }
    println!("{}",x);

}
