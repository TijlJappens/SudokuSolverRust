use sudoku_solver_rust::guess_list::{self, GuessList};
use sudoku_solver_rust::sudoku::Sudoku;
use sudoku_solver_rust::bool_array::BoolArray;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;
use std::error::Error;

static mut moves : u64 = 0;

fn itterative_solver(sudoku : Sudoku, guess_list : &mut GuessList) -> (GuessList,Sudoku){
    loop {
        let mut dummy_sudoku = sudoku.clone();
        dummy_sudoku.insert_guesslist(guess_list);
        dummy_sudoku.update_possibilities();
        while dummy_sudoku.fill_single_possibility()==true {};

        let solved : bool = dummy_sudoku.check_fully_solved();
        let consistent : bool = dummy_sudoku.check_consistent();
        let possible : bool = dummy_sudoku.check_possible();

        if solved && consistent{
            return (guess_list.clone(),dummy_sudoku);
        }else if !consistent || !possible {
            unsafe{moves+=1;}
            guess_list.change_wrong_guess();
        }else{
            guess_list.specify_guess();
        }
    }
} 

fn sudoku_reader(args : Vec<String>) -> Result<Sudoku, Box<dyn Error>> {
    
    if args.len() <= 1 {
        eprintln!("No path to sudoku given.");
        process::exit(1);
    }

    let path: &String = &args[1];
    let mut file: File = File::open(path)?;

    let mut a = [0; 9*9];
    let mut buffer = [0; 1];  // temporary buffer
    let mut i = 0;
    while file.read(&mut buffer)? > 0 {
        let mychar = buffer[0] as char;
        if mychar.is_digit(10) {
            if i >= 9*9 {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Too many numbers in file.")));
            }
            a[i] = mychar.to_digit(10).unwrap() as u8;
            i += 1;
        }
    }
    Ok(Sudoku::new(a))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    print!("url: {}",args[1]);

    println!("\nBefore filling in single possibilities:");
    let mut sudoku = sudoku_reader(args)?;

    println!("{}", sudoku);

    println!("Consistent: {}", sudoku.check_consistent());

    sudoku.update_possibilities();
    let mut guess_list = GuessList::new(sudoku);
    let (end_guesslist, end_sudoku) = itterative_solver(sudoku, &mut guess_list);

    println!("{}", end_guesslist);
    println!("{}", end_sudoku);

    return Ok(());
}
