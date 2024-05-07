use std::fmt;
use std::fs::read;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// A struct representing a sudoku board.
pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    /// Load a sudoku board from a file
    pub fn load(file: &str) -> std::io::Result<Self> {
        let f = File::open(file)?;
        let reader = BufReader::new(f);

        let mut new_board = [[0; 9]; 9];

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            for (j, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    new_board[i][j] = c.to_digit(10).unwrap() as u8;
                }
            }
        }
        
        Ok(Sudoku { board: new_board })
     }

    /// Solve the sudoku board (in-place)
    pub fn solve(&mut self) { 
        self.solve_helper();
    }

    fn get_possible(&self, i: usize, j: usize) -> Vec<u8> {
        let mut possible_nums = vec![];
        let row = self.board[i];
        let col = self.board.iter().map(|row| row[j]);
        let mut block = vec![];

        let i_block = i / 3;
        let j_block = j / 3;

        for i in i_block * 3..(i_block + 1) * 3 {
            for j in j_block * 3..(j_block + 1) * 3 {
                block.push(self.board[i][j]);
            }
        }

        for num in 1..=9 {
            if !row.contains(&num) && !col.clone().any(|x| x == num) && !block.contains(&num) {
                possible_nums.push(num);
            }
        }

        possible_nums
    }

    fn solve_helper(&mut self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    let possible_nums = self.get_possible(i, j);
                    for num in possible_nums {
                        self.board[i][j] = num;
                        if self.solve_helper() {
                            return true;
                        }
                        self.board[i][j] = 0;
                    }
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for (i, rowchunk) in self.board.chunks(3).enumerate() {
            for row in rowchunk.iter() {
                for block in row.chunks(3) {
                    write!(f, "│ ")?;
                    for &num in block.iter() {
                        if num == 0 {
                            write!(f, "  ")?;
                        } else {
                            write!(f, "{} ", num)?;
                        }
                    }
                }
                writeln!(f, "│")?;
            }
            if i != 2 {
                writeln!(f, "├───────┼───────┼───────┤")?;
            }
        }
        write!(f, "└───────┴───────┴───────┘")
    }
}
