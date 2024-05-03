use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::{File, FileType};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{PathBuf};
use serde::Serialize;
use crate::state::point::Point;
use crate::state::State;

pub mod state;

pub fn create_state(path: &PathBuf) -> State{
    let mut f = File::open(path);
    let mut state =State::new(false,false);
    let reader = BufReader::new(f.unwrap());
    let mut m = 0usize;
    let mut p = 0usize;
    for line in reader.lines(){
        let mut n = 0usize;
        let mut store = String::from(line.unwrap());
        if store.contains("X"){
            for c in store.chars(){
                match c{
                    'S' => {state.start = Some(Point::new(m, n));
                        state.cleaned.push(Point::new(m,n));},
                    ' ' => {state.uncleaned.push(Point::new(m,n));},
                    'P' => {state.portals.push(Point::new(m,n));},
                    _ => (),
                }
                n = n + 1usize;
            }
            m = m + 1usize;
        }else {
            if store.trim() != "FIND PLAN" && store.trim() != "CHECK PLAN"{
                state.moves = Some(store);
            }else{
                if store.trim() == "FIND PLAN"{
                    state.find = true;
                }else{
                    state.check = true;
                }
            }
        }
    }


    state
}

pub fn process_state_start_given(state: &mut State){
    if state.check{
        if state.moves != None{
            let str = state.moves.clone().unwrap();
            for i in str.chars(){
                state.move_cleaner(i);
            }
        }
    }else{
        state.find_plan();
    }
}

pub fn process_state_start_not_given(st: &mut State) -> HashSet<Point>{
    let mut result = HashSet::new();
    let unclean = st.uncleaned.clone();
    for i in unclean{
        let mut state = st.clone();
        state.start = Some(i.clone());
        state.uncleaned.remove(state.uncleaned.binary_search(&i).unwrap());
        state.cleaned.push(i);
        let str = state.moves.clone().unwrap();
        for i in str.chars() {
            state.move_cleaner(i);
        }
        result.extend(state.uncleaned.clone());
    }


    result
}


pub fn write_to_file_start_given(state: State, path: &mut PathBuf,filename: &str) -> std::io::Result<()>{
    create_dir_sol(path);
    path.push(filename);
    let f = File::create(path)?;
    let mut buffer = BufWriter::new(f);
    if state.check{
        if state.uncleaned.is_empty(){
            writeln!(&mut buffer,"GOOD PLAN")?;
        }else{
            writeln!(&mut buffer,"BAD PLAN")?;
            for i in state.uncleaned.clone(){
                writeln!(&mut buffer,"{}, {}",i.y,i.x)?;
            }
        }
    }else{
        writeln!(&mut buffer,"{}",state.moves.unwrap())?;
    }
    Ok(())
}

pub fn write_to_file_start_not_given(set:HashSet<Point>, path :&mut PathBuf, filename: &str) -> std::io::Result<()>{
    create_dir_sol(path);
    path.push(filename);
    let f = File::create(path)?;
    let mut buffer = BufWriter::new(f);
    if set.is_empty(){
        writeln!(&mut buffer,"GOOD PLAN")?;
    }else{
        writeln!(&mut buffer,"BAD PLAN")?;
        for i in set{
            writeln!(&mut buffer,"{}, {}",i.y,i.x)?;
        }
    }

    Ok(())
}

fn create_dir_sol(path: &mut PathBuf){
    path.push("solutions");
    if !path.exists(){
        fs::create_dir(path).unwrap();
    }
}

pub fn directory_parser(path: &mut PathBuf){
    if path.is_dir(){
        for contents in path.read_dir().expect("Cannot read directory"){
            let mut p = contents.unwrap().path();
            if p.is_file(){
                let mut state = create_state(&mut p);
                let file_name = &p.file_name().unwrap().clone().to_str().unwrap().replace("problem","solution");
                if state.start != None{
                    process_state_start_given(&mut state);
                    write_to_file_start_given(state, &mut path.clone(),file_name).unwrap()
                }else{
                    let set = process_state_start_not_given(&mut state);
                    write_to_file_start_not_given(set,&mut path.clone(),file_name).unwrap()
                }
            }
        }
    }
}



pub struct ElevateMap{
    pub map: Vec<Speicher>,
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
                speicher.uncleaned.extend(uncleaned);
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



pub struct Speicher{
    pub start: Point,
    pub uncleaned: Vec<Point>,
}
impl Display for Speicher{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{},{}",self.start,self.uncleaned.len())
    }
}
impl Speicher{
    pub fn new(x: usize,y:usize) -> Self{
        Speicher{
            start:Point::new(x,y),
            uncleaned: Vec::new(),
        }
    }

    pub fn change_start(&mut self, point: Point){
        self.start = point;
        if self.uncleaned.contains(&point){
            self.uncleaned.remove(self.uncleaned.binary_search(&point).unwrap());
        }
    }
}