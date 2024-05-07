use std::path::{PathBuf};
use aisysproj::{create_state, directory_parser, ElevateMap};
use crate::state::point::Point;

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("example-problems/problem_f_19.txt");
    let mut dir = create_state(&path);
    let mut el = ElevateMap::create(&mut dir);
    let str = el.unwrap().find_plan();
    println!("{}",str);
    let br  = "ENWNWSWSWSWWNWNENEEENWWWNWSWSSESEEEEENNNESESEWWSSWSWNWNNENENWNEEEESWWSESSEWWWSWWWWNWWNNNENESSEESENNWNNWNWNSSESSNEEEWWWWWWWNNWNSWWSSSSSNESESESNNEEWNWNWWSNWEENWWNEEESSEEESSNNESEEWSWNWNNEWSEESWWNWWWWWWWS
";
    let sr = "SESESEEEEEEWWWWWWNENEESEEEEEWNWWWNEEENWWWWSWWWWSSNWNEENEENEEEEEENSWSSSSEWWSWWWWWWNENEESEENENSWWWNWWWWSSNWNEENEEENENEEENSWWNSWWWSWNN";
}

