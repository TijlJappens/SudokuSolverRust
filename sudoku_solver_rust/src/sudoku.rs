use crate::bool_array::BoolArray;
use crate::guess_list::GuessList;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Sudoku {
    pub table : [u8;9*9],
    pub possibilities : [BoolArray;9*9],
}
impl Sudoku {
    pub fn new(t : [u8;9*9]) -> Self {
        let mut possibilities = [BoolArray::new_from_bool(false); 9*9];
        for i in 0..9 {
            for j in 0..9 {
                if t[i + 9*j] != 0 {
                    possibilities[i + 9*j] = BoolArray::new_from_bool(false);
                } else {
                    possibilities[i + 9*j] = BoolArray::new_from_bool(true);
                }
            }
        }
        Self {
            table: t,
            possibilities,
        }
    }
    pub fn new_empty() -> Self {
        return Self::new([0;9*9]);
    }
    fn change_element(&mut self, i : usize, j : usize, value : u8){
        self.table[i + 9*j] = value;
    }
    pub fn get_element(&self, i : usize, j : usize) -> u8{
        return self.table[i + 9*j];
    }
    pub fn get_possibilities(&self, i : usize, j : usize) -> BoolArray{
        return self.possibilities[i + 9*j];
    }

    //Checks if the table is consistent.
    pub fn check_consistent(&self) -> bool{
        //For all columns, rows and squares, for all numbers,
        // check if a number isn't included twice
        for i in 0..9 {
            for n  in 1..10 {
                let mut n_count_row : u8 = 0;
                let mut n_count_column : u8 = 0;
                let mut n_count_square : u8 = 0;
                for j in 0..9 {
                    //This checks columns.
                    if self.get_element(i,j)==n {n_count_row+=1;}
                    //This checks rows.
                    if self.get_element(j,i)==n {n_count_column+=1;}
                    //This checks squares.
                    if self.get_element((i*3)%9+j%3,3*((i*3)/9)+j/3)==n {n_count_square+=1;}
                }
                if n_count_row>1 || n_count_column>1 || n_count_square>1 {return false;}
            }
        }
        return true;
    }

    fn check_consistent_single_position(&self, x: usize  , y: usize) -> bool{
        let n = self.get_element(x, y);
        let mut n_count_row : u8 = 0;
        let mut n_count_column : u8 = 0;
        let mut n_count_square : u8 = 0;
        for j in 0..9 {
            //This checks columns.
            if self.get_element(x,j)==n {n_count_row+=1;}
            //This checks rows.
            if self.get_element(j,y)==n {n_count_column+=1;}
            //This checks squares.
            let  i = (x/3+3*(y/3))*3;
            if self.get_element(i%9+j%3,3*(i/9)+j/3)==n {n_count_square+=1;}
        }
        if n_count_row>1 || n_count_column>1 || n_count_square>1 {
            return false;
        }else{
            return true;
        }
    }

    pub fn check_fully_solved(&self) -> bool{
        for i in 0..9*9 {
            if self.table[i]==0 {return false;}
        }
        return true;
    }

    fn update_single_possibility(&mut self, i : usize, j : usize){
        if self.get_element(i, j)!=0 {self.possibilities[i+9*j].set_all_false();}
        else{
            for n in 1..10 {
                self.change_element(i, j, n);
                if self.check_consistent_single_position(i, j)==false {
                    self.possibilities[i+9*j].set_false(n as usize);
                }
                self.change_element(i,j,0); 
            }
        }
    }

    pub fn update_possibilities(&mut self){
        for i in 0..9 {
            for j in 0..9 {
                self.update_single_possibility(i, j);
            }
        }
    }

    pub fn check_possible(&self) -> bool{
        for i in 0..9 {
            for j in 0..9 {
                if self.get_element(i, j)==0 && self.get_possibilities(i, j).none_true() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn fill_single_possibility(&mut self) -> bool{
        let mut changed = false;
        for i in 0..9 {
            for j in 0..9 {
                if self.get_element(i, j)==0 {
                    if self.get_possibilities(i, j).only_one_true()==true {
                        self.change_element(i, j, self.get_possibilities(i, j).first_true() as u8);
                        changed=true;
                    }else if self.get_possibilities(i, j).none_true()==true {
                        return false;
                    }
                }
            }
        }
        self.update_possibilities();
        return changed;
    }

    pub fn insert_guesslist(&mut self, guesslist : &GuessList){
        for t in &guesslist.guesses {
            self.change_element(t.first_x as usize, t.first_y as usize, t.second);
        }
    }

}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let mut s = String::new();
        for i in 0..9{
            if i%3==0 {
                for _ in 0..8*3+1 {
                    s.push_str("-");
                }
                s.push_str("\n");
            }
            for j in 0..9{
                if j%3==0 {s.push_str("| ");}
                s.push_str(&format!("{} ", self.table[j + 9 * i]));
            }
            s.push_str("|\n");
        }
        for _ in 0..8*3+1 {
            s.push_str("-");
        }
        s.push_str("\n");
        match write!(f, "{}", s.as_str()){
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}