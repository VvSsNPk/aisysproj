use std::fmt::{Display, Formatter};
use crate::state::point::Point;
use crate::state::State;

pub struct ElevateMap{
    map: Vec<Speicher>,
}

impl ElevateMap{
    pub fn new() -> Self{
        ElevateMap{
            map : Vec::new(),
        }
    }

    pub fn create(state: &mut State) -> Option<Self>{
        if state.start == None {
            let mut map = ElevateMap::new();
            let mut uncleaned = state.uncleaned.clone();
            for i in uncleaned{
                let mut speicher = Speicher::new(i.x, i.y);
                let mut uncleaned = state.uncleaned.clone();
                uncleaned.remove(state.uncleaned.binary_search(&i).unwrap());
                speicher.cleaned.extend(uncleaned);
                map.map.push(speicher);
            }
            return Some(map)
        }else {
            None
        }
    }
}

impl Display for ElevateMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.map.len())
    }
}



struct Speicher{
    start: Point,
    cleaned: Vec<Point>,
}
impl Display for Speicher{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{},{}",self.start,self.cleaned.len())
    }
}
impl Speicher{
    pub fn new(x: usize,y:usize) -> Self{
        Speicher{
            start:Point::new(x,y),
            cleaned: Vec::new(),
        }
    }
}