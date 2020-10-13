use sudoku::Sudoku;
use std::fs;
use structopt::StructOpt;
use std::path::Path;

fn read_sudoku_from_file(filename : &Path) -> Sudoku {
    let contents = fs::read_to_string(filename).expect(&format!("Could not read {}", filename.to_str().expect("no path given")));
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

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let mut sudoku = read_sudoku_from_file(args.path.as_path());

    println!("Original Sudoku: {}", sudoku);
    sudoku.solve();
    println!("Solved Sudoku: {}", sudoku);
}
