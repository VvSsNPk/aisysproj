use std::cmp::PartialEq;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};
use std::usize::MAX;
use crate::state::point::Point;

pub mod point;
#[derive(Clone)]
pub struct State{
    pub start: Option<Point>,
    pub cleaned: HashSet<Point>,
    pub uncleaned: HashSet<Point>,
    pub portals: HashSet<Point>,
    pub moves : Option<String>,
    pub check: bool,
    pub find: bool,
}

impl  State {
    pub fn new(check: bool, find: bool) -> Self{
        State{
            start: None,
            cleaned: HashSet::new(),
            uncleaned: HashSet::new(),
            portals: HashSet::new(),
            moves: None,
            check,
            find,
        }
    }
}

impl Display for State{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"start: {:?}\ncleaned: {},uncleaned: {}\nportals: {:?}\nmoves: {:?}\ncheck: {}, find: {}",
                    self.start.unwrap(), self.cleaned.len(),self.uncleaned.len(), self.portals,self.moves, self.check, self.find)
    }
}



impl State {
    pub fn move_cleaner(&mut self, c: char) {
        if self.start != None {
            let point = self.start.unwrap().clone();
            if c == 'N' || c == 'S' {
                if point.x != 0usize && point.x != 11usize {
                    match c {
                        'N' => self.checker(Point { x: point.x - 1usize, y: point.y }),
                        'S' => self.checker(Point { x: point.x + 1usize, y: point.y }),
                        _ => (),
                    }
                }
            } else if c == 'W' || c == 'E' {
                if point.y != 0usize && point.y != 17usize {
                    match c {
                        'W' => self.checker(Point { x: point.x, y: point.y - 1usize }),
                        'E' => self.checker(Point { x: point.x, y: point.y + 1usize }),
                        _ => (),
                    }
                }
            }
        }
    }


    pub fn checker(&mut self, point: Point) {
        if !self.portals.is_empty() {
            if self.portals.contains(&point) {
                let mut m = Point { x: 0usize, y: 0usize };
                for i in &self.portals {
                    if i != &point {
                        m.x = i.x;
                        m.y = i.y;
                    }
                }
                self.start = Some(m);
            }
        } else {
            if self.uncleaned.contains(&point) {
                self.start = Some(point);
                self.cleaned.insert(point);
                self.uncleaned.remove(&point);
            } else {
                if self.cleaned.contains(&point) {
                    self.start = Some(point);
                }
            }
        }
    }

    pub fn heuristics(&self) -> usize{
        self.uncleaned.len()
    }

    pub fn is_goal(&self) -> bool{
        self.uncleaned.is_empty()
    }

    pub fn get_neighbours(&self) -> Vec<Store>{
        let mut result = Vec::new();
        if self.start != None{
            for i in "NEWS".chars(){
                let mut clone = self.clone();
                clone.move_cleaner(i);
                result.push(Store::new(i,clone.heuristics()))
            }
        }
        result.sort_by(|a,b|a.y.cmp(&b.y));
        result
    }
}

#[derive(Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Store{
    x:char,
    y:usize,
}

impl Store {
    pub fn new(x:char,y:usize) -> Self{
        Store{
            x,y,
        }
    }
}

impl State{
    pub fn find_plan(&self)-> Vec<char>{
        let mut result = Vec::new();
        let mut clone = self.clone();
        while !clone.uncleaned.is_empty(){
            let mut x = usize::MAX;
            let mut c = ' ';
            for i in "NEWS".chars(){
                let mut clone2 = clone.clone();
                clone2.move_cleaner(i);
                if clone2.uncleaned.len() < x{
                    x = clone2.uncleaned.len();
                    clone = clone2;
                    c = i;
                }
            }
            result.push(c);
        }

        result
    }
}
