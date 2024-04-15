use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// A struct representing a sudoku board.
pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    /// Load a sudoku board from a file
    pub fn load(file: &str) -> std::io::Result<Self> { }

    /// Solve the sudoku board (in-place)
    pub fn solve(&mut self) { }

    fn get_possible(&self, i: usize, j: usize) -> Vec<u8> {
        vec![]
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
