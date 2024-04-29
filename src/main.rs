use std::path::Path;
use aisysproj::create_state;

pub mod state;

fn main() {
    let path = Path::new("problem_d_03.txt");
    let mut x = create_state(path);
    println!("{}\n",x);
/*    let mut str = match &x.moves{
        None => panic!("No string"),
        Some(x) => x.clone(),
    };
    for i in str.chars(){
        x.move_cleaner(i);
    }
    println!("{}",x);
    */
    let j = x.find_plan();
    println!("{:?}",j);
}
