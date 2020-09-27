use std::fmt;

pub struct Sudoku {
    puzzle : Vec<u8>,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("\n");
        
        for i in 0..9 {
            if i % 3 == 0 && i > 0 {
                result.push_str("----+---+----\n")
            }

            result.push_str("|");
            for j in 0..3 {
                for c in &self.puzzle[9*i..9*i+9][j*3..j*3+3] {
                    result.push_str(&c.to_string());
                }
                result.push_str("|");
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

impl Sudoku {
    pub fn new(puzzle: Vec<u8>) -> Sudoku {
        Sudoku{puzzle}
    }

    pub fn solve(&mut self) -> bool {
        if self.is_solved() {
            return true
        }

        for row in 0..9 {
            for col in 0..9 {
                if *self.get(row, col) == 0 {
                    for x in 1..10 {
                        if self.is_allowed(row, col, x) {
                            self.update(row, col, x);                           
                            if self.solve() {
                                return true
                            } else{
                                self.update(row, col, 0);
                            }
                        } 
                    }
                    return false;
                }
            }
        }
        return false
    }
    
    fn is_solved(&self) -> bool {
        self
        .puzzle
        .iter()
        .find(|y| **y == 0) 
        .is_none()
    }

    fn update(&mut self, row:usize, col:usize, x:u8) -> () {
        self.puzzle[row*9 + col] = x;
    }

    fn is_allowed(&self, row:usize, col:usize, x:u8) -> bool {
        self
        .row(row)
        .iter()
        .chain(self.col(col).iter())
        .chain(self.square(row/3, col/3).iter())
        .find(|y| **y == x)
        .is_none()
    }

    fn get(&self, row:usize, col: usize) -> &u8 {
        &self.puzzle[row*9 + col]
    }

    fn row(&self, row: usize) -> Vec<u8> {
        self.puzzle[row*9..row*9+9].to_vec()
    }

    fn col(&self, col: usize) -> Vec<u8> {
        let mut column : Vec<u8> = Vec::new();
        for r in 0..9 {
            column.push(self.puzzle[r*9 + col]);
        }
        return column
    }

    fn square(&self, square_row : usize, square_col : usize) -> Vec<u8> {
        let mut square : Vec<u8> = Vec::new();
        for r in 0..3 {
            for c in 0..3{
                square.push(self.puzzle[square_row * 27 + r * 9 + square_col * 3 + c]);
            }
        }
        return square
    }
}
