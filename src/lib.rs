use std::any::Any;
use std::fs;
use std::fs::{File, FileType};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use crate::state::point::Point;
use crate::state::State;

pub mod state;

pub fn create_state(path: &Path) -> State{
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
                        state.cleaned.insert(Point::new(m,n));},
                    ' ' => {state.uncleaned.insert(Point::new(m,n));},
                    'P' => {state.portals.insert(Point::new(m,n));},
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