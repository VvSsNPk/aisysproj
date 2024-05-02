use std::any::Any;
use std::collections::HashSet;
use std::fs;
use std::fs::{File, FileType};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
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


pub fn write_to_file(state: State, path: &mut PathBuf,filename: &str){
    let mut p = path.clone();
    p.push(filename);
    let f = File::create(p)?;
    let mut buffer = BufWriter::new(f);
    if state.check{
        if state.uncleaned.is_empty(){
            writeln!(&mut buffer,"GOOD PLAN")?;
        }else{
            writeln!(&mut buffer,"BAD PLAN")?;
        }
    }
}