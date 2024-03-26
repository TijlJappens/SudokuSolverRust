use std::fmt;

#[derive(Copy, Clone)]
pub struct BoolArray {
    bool_array : [bool; 9],
    amount_true : u8,
}

impl BoolArray {
    pub fn new() -> Self {
        BoolArray {
            bool_array: [false; 9],
            amount_true: 0,
        }
    }
    pub fn new_from_bool(b : bool) -> Self {
        BoolArray {
            bool_array: [b; 9],
            amount_true: if b {9}else{0},
        }
    }
    pub fn first_true(&self) -> u8 {
        for i in 0..9 {
            if self.bool_array[i]==true {return (i as u8).wrapping_add(1);}
        }
        panic!("Accessing first true element of an array that is all false.")
    }
    pub fn none_true(&self) -> bool{
        if self.amount_true==0 {return true;}
        else{return false;}
    }
    pub fn only_one_true(&self) -> bool{
        if self.amount_true==1 {return true;}
        else{return false;}
    }
    pub fn get_bool(&self,i : usize) -> bool{
        return self.bool_array[i-1];
    }
    pub fn set_false(&mut self, i : usize){
        if self.bool_array[i-1]==true {self.bool_array[i-1]=false;self.amount_true-=1;}
    }
    pub fn set_all_false(&mut self){
        if self.amount_true>0 {self.bool_array=[false;9];}
    }
}

impl fmt::Display for BoolArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for i in 0..9{
            match write!(f,"| {} ",self.bool_array[i]){
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        match write!(f,"|"){
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        Ok(())
    }
}