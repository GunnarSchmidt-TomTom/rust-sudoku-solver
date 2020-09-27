use sudoku::Sudoku;
use std::env;
use std::fs;

fn read_sudoku_from_file(filename : &String) -> Sudoku {
    let contents = fs::read_to_string(filename).expect(&format!("Could not read {}", filename));
    let lines = contents.lines();

    let mut numbers : Vec<u8> = Vec::new();

    for line in lines {
        if line.starts_with("-") {
            continue;
        }

        for c in line.chars() {
            if c.is_ascii_digit(){
                let num = c.to_digit(10).expect(&format!("Unexpected character {}",c));
                numbers.push(num as u8); // singe digits always fit into u8
            } else if c == '|' {
                // ignore pipes
            }
            else {
                panic!("Character '{}' not expected", c);
            }
        }
    }
    
    if numbers.len() == 81 {
        return Sudoku::new(numbers)
    } else {
        panic!("Expexted 9x9 Sudoku, but got \n{}", contents);
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut sudoku = read_sudoku_from_file(&args[1]);

    println!("Original Sudoku: {}", sudoku);
    sudoku.solve();
    println!("Solved Sudoku: {}", sudoku);
}
