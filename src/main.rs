use std::path::Path;
use aisysproj::create_state;

pub mod state;

fn main() {
    let path = Path::new("problem_e_19.txt");
    let mut x = create_state(path);
    println!("{}",x);
    for i in "SEESESWWNWSSSEESWWNWNNNNNNEEESESSWNNENNNNEEWWNWSSSSWNNNEEEESEEN".chars(){
        x.move_cleaner(i);
    }
    println!("{}",x)

}
