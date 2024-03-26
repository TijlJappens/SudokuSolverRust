use crate::bool_array::BoolArray;
use crate::sudoku::Sudoku;

use std::fmt;

#[derive(Copy, Clone)]
pub struct Triple {
    pub first_x: usize,
    pub first_y: usize,
    pub second: u8,
    pub third: BoolArray,
}

impl Triple {
    pub fn new(i: usize, j: usize, k: u8, l: BoolArray) -> Self {
        Self {
            first_x: i,
            first_y: j,
            second: k,
            third: l,
        }
    }

    pub fn is_contained(&self) -> bool {
        if self.third.get_bool(self.second as usize) {
            return true;
        }else{
            return false;
        }
    }

    pub fn get_lowest(&self) -> u8 {
        self.third.first_true()
    }
}

#[derive(Clone)]
pub struct GuessList {
    pub guesses: Vec<Triple>,
}

impl GuessList {
    pub fn new(s : Sudoku) -> Self {
        let mut guesses : Vec<Triple> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if s.get_element(i,j)==0 {
                    let a = s.get_possibilities(i,j);
                    let t = Triple::new(i,j,0,a);
                    guesses.push(t);
                }
            }
        }
        GuessList {
            guesses,
        }
    }
    fn get_first_zero_index(&self) -> usize {
        for i in 0..self.guesses.len() {
            if self.guesses[i].second==0 {
                return i;
            }
        }
        panic!("No zeros in guesslist.");
    }

    pub fn specify_guess(&mut self) {
        let to_be_specified = self.get_first_zero_index();
        self.guesses[to_be_specified].second = self.guesses[to_be_specified].get_lowest();
    }

    pub fn change_wrong_guess(&mut self) {
        let i = self.get_first_zero_index()-1;
        self.guesses[i].second +=1 ;
        while self.guesses[i].second<=9 {
            if self.guesses[i].is_contained() {return;}
            self.guesses[i].second +=1;
        }
        self.guesses[i].second = 0;
        self.change_wrong_guess();
    }
}

impl fmt::Display for GuessList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "| ")?;
        for t in &self.guesses {
            write!(f, "{} |", t.second)?;
        }
        Ok(())
    }
}
