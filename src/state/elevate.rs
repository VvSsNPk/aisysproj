use crate::state::point::Point;

struct ElevateMap{
    map: Vec<Speicher>,
}

impl ElevateMap{
    pub fn new() -> Self{
        ElevateMap{
            map : Vec::new(),
        }
    }
}


struct Speicher{
    start: Point,
    cleaned: Vec<Point>,
}

impl Speicher{
    pub fn new(x: usize,y:usize) -> Self{
        Speicher{
            start:Point::new(x,y),
            cleaned: Vec::new(),
        }
    }
}