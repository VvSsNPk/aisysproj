use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::state::point::Point;

pub mod point;

#[derive(Clone, Eq, PartialEq, Debug,Hash,Ord, PartialOrd)]
pub struct State {
    pub start: Option<Point>,
    pub cleaned: Vec<Point>,
    pub uncleaned: Vec<Point>,
    pub portals: Vec<Point>,
    pub moves: Option<String>,
    pub check: bool,
    pub find: bool,
}

impl State {
    pub fn new(check: bool, find: bool) -> Self {
        State {
            start: None,
            cleaned: Vec::new(),
            uncleaned: Vec::new(),
            portals: Vec::new(),
            moves: None,
            check,
            find,
        }
    }
}


impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}\ncleaned: {},uncleaned: {}\nportals: {:?}\nmoves: {:?}\ncheck: {}, find: {}",
               self.start.unwrap(), self.cleaned.len(), self.uncleaned.len(), self.portals, self.moves, self.check, self.find)
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
         if self.portals.contains(&point) {
             if !self.portals.is_empty(){
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
                self.cleaned.push(point);
                self.uncleaned.remove(self.uncleaned.binary_search(&point).unwrap());
            } else {
                if self.cleaned.contains(&point) {
                    self.start = Some(point);
                }
            }
        }
    }

    pub fn heuristics(&self) -> usize {
        self.uncleaned.len()
    }

    pub fn is_goal(&self) -> bool {
        self.uncleaned.is_empty()
    }

    pub fn get_neighbours(&self, mut s:String) -> Vec<(String, State)> {
        let mut result = Vec::new();
        let mut sol = Vec::new();
        for i in "SEWN".chars() {
            let mut clone = self.clone();
            clone.move_cleaner(i);
            if !result.contains(&clone) && &clone != self {
                result.push(clone.clone());
                let mut y = s.clone();
                y.push(i);
                sol.push((y,clone));
            }
        }

        sol
    }
}


impl State {
    pub fn find_plan(&mut self) -> String {
        let mut map = VecDeque::new();
        map.push_back((String::new() ,self.clone()));
        loop{
            let mut x = match map.pop_front(){
                Some(Y) => Y,
                None => break,
            };
            let (z,i) = x;
            if i.is_goal(){
                return z;
            }
            for st in i.get_neighbours(z){
                //println!("{:?}",st);
                map.push_back(st);
            }
            let mut vec:Vec<_> = map.drain(..).collect();
            vec.sort_by(|(l,a),(m,b)|a.uncleaned.len().cmp(&b.uncleaned.len()));
            map = vec.into();

        }
        String::new()
    }
}
